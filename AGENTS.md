# Tinta: instruções para agentes e contribuidores

Tinta é um editor de PDF de código aberto, escrito em Rust. Roda nativamente em macOS,
Windows, Linux e FreeBSD, e no navegador via WebAssembly. É um fork do projeto PdfCraft
(MIT OR Apache-2.0), com a marca e os links da comunidade original removidos. Os créditos
ao projeto original estão em `NOTICE` e `ATTRIBUTION.md`.

Estas regras valem para qualquer contribuição. Os números de seção (§) são citados no código.

## 1. Ativos e licenças

### 1.1 Dados técnicos da Adobe
Nada derivado de produtos Adobe pode entrar no repositório: nem ícones, nem fontes, nem
imagens, nem dados extraídos de instalações do Acrobat, Photoshop ou similares. Dados técnicos
publicados pela Adobe só entram na lista fechada de exceções em `xtask/src/assets.rs`
(`ADOBE_DATA_ALLOWED`).

### 1.2 Marcas
Este fork não contém marcas de terceiros. O nome "Tinta" e o ícone são do projeto. Logos e
nomes de outros projetos não podem ser adicionados.

### 1.3 Licenças de ativos
Todo ativo (ícone, fonte, imagem, ícone de interface) precisa de licença aberta compatível
com MIT/Apache-2.0 e de uma entrada em `ATTRIBUTION.toml`. `cargo xtask assets` verifica isso.

### 1.4 Fontes
Fontes não são versionadas neste repositório. A fonte opcional `craft-fonts`
(https://github.com/storytold/craft-fonts, licença OFL) pode ser usada via `CRAFT_FONTS_DIR`.
O código que usa `pdfcraft_fonts::CRAFT_FONTS` precisa funcionar quando a pasta estiver vazia.

## 2. Clean-room

O comportamento vem de especificações públicas (ISO 32000-2) e de observação de caixa-preta
de arquivos próprios. Não copie código de outras implementações de PDF. Bibliotecas com licença
GPL ou AGPL não podem ser incorporadas; podem ser usadas apenas como processo externo de teste,
se isso for documentado.

## 3. Automação e servidor MCP

O CLI (`tinta-cli`) e o servidor MCP são opcionais e nunca abrem portas de rede por padrão.
Todo caminho de arquivo que um agente pode ler ou escrever deve ficar dentro do `--root`
informado. Ferramentas novas precisam de um equivalente no registro de automação.

## 4. Nunca travar

Todo PDF, script, chamada MCP, arquivo de configuração e tecla pressionada é entrada não
confiável. Nenhuma entrada pode derrubar o app ou fazer o usuário perder trabalho.

- Em código fora de testes, não use `unwrap`, `expect`, `panic!`, `unreachable!` ou `todo!`,
  salvo quando o caso for provadamente impossível, com comentário explicando por quê.
- Não use indexação ou fatiamento com posições vindas da entrada. Use `get` e fatie strings
  apenas em limites de caractere.
- Use aritmética verificada ou saturada com números vindos da entrada, e trate divisão por
  zero e NaN/inf.
- Limite alocações dimensionadas pela entrada e recursões com profundidade máxima.
- Não use `unsafe` (`unsafe_code = "forbid"`).
- Cada correção de travamento precisa de um teste de regressão com dados sintéticos.

## 5. Qualidade antes de cada commit

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` nos crates alterados
- `cargo xtask layers` e `cargo xtask assets`

Escopo: rode builds e testes só nos crates que você alterou (`-p <crate>`). A compilação
completa do workspace é responsabilidade do CI.

## 6. Histórico e commits

- `main` é a única branch de integração. Trabalhe em branches `<tipo>/<assunto>`
  (`feat/`, `fix/`, `docs/`, `chore/`, `refactor/`).
- Um commit por mudança coerente, com mensagem em português ou inglês, descrevendo o motivo.
- Não faça push direto na `main`.
