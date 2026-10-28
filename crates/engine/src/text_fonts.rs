//! Explicit document-font discovery. Interface fallbacks never enter this catalogue.
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FontFile {
    pub path: PathBuf,
    pub family: String,
    pub face: String,
}

/// Standard user-installed font folders; discovery is performed only on request.
pub fn installed_directories() -> Vec<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    if cfg!(target_os = "macos") {
        let mut dirs = vec![PathBuf::from("/Library/Fonts")];
        if let Some(home) = home {
            dirs.push(home.join("Library/Fonts"));
        }
        dirs
    } else if cfg!(windows) {
        let mut dirs = std::env::var_os("WINDIR").map(|p| PathBuf::from(p).join("Fonts")).into_iter().collect::<Vec<_>>();
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            dirs.push(PathBuf::from(local).join("Microsoft/Windows/Fonts"));
        }
        dirs
    } else {
        let mut dirs = vec![PathBuf::from("/usr/share/fonts"), PathBuf::from("/usr/local/share/fonts")];
        if let Some(home) = home {
            dirs.extend([home.join(".local/share/fonts"), home.join(".fonts")]);
        }
        dirs
    }
}

/// Bounded, symlink-free scan. Only standalone, openly licensed, embeddable faces are listed.
/// The caller controls roots (automation resolves confinement before calling this).
pub fn discover(roots: &[PathBuf]) -> Vec<FontFile> {
    use std::io::Read;
    let mut pending: Vec<_> = roots.iter().take(16).map(|r| (r.clone(), 0usize)).collect();
    let mut files = Vec::new();
    let mut entries = 0usize;
    let mut budget = 64u64 << 20;
    let mut seen = std::collections::HashSet::new();
    while let Some((path, depth)) = pending.pop() {
        if entries >= 4096 || budget == 0 {
            break;
        }
        entries += 1;
        let Ok(meta) = std::fs::symlink_metadata(&path) else { continue };
        if meta.file_type().is_symlink() || !seen.insert(path.clone()) {
            continue;
        }
        if meta.is_dir() && depth < 4 {
            if let Ok(children) = std::fs::read_dir(&path) {
                let remaining = 4096usize.saturating_sub(entries.saturating_add(pending.len()));
                pending.extend(children.take(remaining).filter_map(Result::ok).map(|p| (p.path(), depth + 1)));
            }
        } else if meta.is_file() && meta.len() <= (16 << 20) && font_extension(&path) {
            if meta.len() > budget {
                continue;
            }
            budget = budget.saturating_sub(meta.len());
            let mut bytes = Vec::new();
            let read = std::fs::File::open(&path).and_then(|f| f.take((16 << 20) + 1).read_to_end(&mut bytes));
            if read.is_err() || tinta_fonts::embedding::validate_document_input(&bytes).is_err() {
                continue;
            }
            if let Ok((family, face)) = tinta_fonts::embedding::names(&bytes) {
                files.push(FontFile { path, family, face });
            }
        }
    }
    files.sort_by(|a, b| a.face.cmp(&b.face).then(a.path.cmp(&b.path)));
    files
}

/// Tinta's own fonts folder. Openly licensed faces put here complete subsets without being
/// installed for the whole system: `~/Library/Application Support/Tinta/Fonts` (macOS),
/// `%LOCALAPPDATA%\Tinta\Fonts` (Windows), `$XDG_DATA_HOME/tinta/fonts` (others).
pub fn tinta_fonts_directory() -> Option<PathBuf> {
    let env = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(target_os = "macos") {
        env("HOME").map(|h| h.join("Library/Application Support/Tinta/Fonts"))
    } else if cfg!(windows) {
        env("LOCALAPPDATA").map(|d| d.join("Tinta").join("Fonts"))
    } else {
        env("XDG_DATA_HOME").or_else(|| env("HOME").map(|h| h.join(".local/share"))).map(|d| d.join("tinta/fonts"))
    }
}

/// Register the openly licensed faces found in `roots` (see [`discover`]) as full faces, so
/// characters typed into a subset of the same family, or of a proprietary family they are
/// metric-compatible with, keep the source look. Returns how many faces were added. Reads files:
/// call it off the UI thread.
pub fn register_full_faces(roots: &[PathBuf]) -> usize {
    use std::io::Read;
    discover(roots)
        .into_iter()
        .filter(|f| {
            let mut bytes = Vec::new();
            std::fs::File::open(&f.path).and_then(|file| file.take((16 << 20) + 1).read_to_end(&mut bytes)).is_ok()
                && tinta_fonts::embedding::register_full_face(bytes).unwrap_or(false)
        })
        .count()
}

/// [`register_full_faces`] over the installed font folders and [`tinta_fonts_directory`].
pub fn register_installed_full_faces() -> usize {
    let mut roots = installed_directories();
    roots.extend(tinta_fonts_directory());
    register_full_faces(&roots)
}

fn font_extension(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("ttf") || e.eq_ignore_ascii_case("otf"))
}
