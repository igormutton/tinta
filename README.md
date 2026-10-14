# Tinta

Editor de PDF de código aberto, escrito em Rust. Lê, organiza, combina, divide, protege e
comenta PDFs em um app rápido e nativo, sem conta, sem telemetria e sem envio de arquivos
para a nuvem.

> **Estado:** alfa. Funciona para tarefas do dia a dia, mas ainda não substitui um editor
> profissional completo. Veja a seção "Limitações conhecidas".

## Origem

Tinta é um fork de [PdfCraft](https://github.com/storytold/pdfcraft), um editor de PDF
reimplementado do zero, a partir de especificações públicas (ISO 32000-2) e observação de
arquivos próprios, sem copiar código de outros programas.

O código original é licenciado sob MIT OR Apache-2.0 e foi escrito pela equipe ArtCraft.
Este fork mantém essas licenças, os avisos de copyright e os créditos em `NOTICE` e
`ATTRIBUTION.md`. As alterações feitas aqui são:

- o nome do projeto e de seus crates foi trocado para Tinta;
- logos, nomes e links da marca ArtCraft foram removidos;
- a verificação de atualizações aponta para este repositório;
- a documentação de contribuição (`AGENTS.md`, `CLAUDE.md`) foi reescrita.

Este projeto não é afiliado, patrocinado nem endossado pela Adobe.

## Como compilar

Requer Rust 1.95 ou superior.

```sh
git clone https://github.com/igormutton/tinta
cd tinta
cargo run --release -p tinta -- arquivo.pdf   # app desktop
cargo test --workspace                        # testes
```

A versão de linha de comando fica em `apps/tinta-cli`:

```sh
cargo run -p tinta-cli -- tools               # lista as ferramentas de automação
```

## Limitações conhecidas

Herdadas do projeto original e ainda não corrigidas neste fork:

- edição confiável de texto existente é limitada, especialmente em CJK e right-to-left;
- não há conversão de Office para PDF;
- OCR funciona bem apenas com alfabeto latino;
- assinaturas com tokens e validação online de longo prazo não estão implementadas;
- formulários XFA não são suportados.

## Licença

Licenciado sob [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), à sua escolha.
Consulte `NOTICE` e `ATTRIBUTION.md` para os créditos e as licenças das dependências e ativos.
