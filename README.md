# secrets-scan-demo

Tiny Rust project used to validate the
[`rechenmaschine-playground/ci-templates`](https://github.com/rechenmaschine-playground/ci-templates)
reusable workflows end-to-end.

## What this demonstrates

- Consumer-side wiring of the `secrets-scan` reusable workflow via
  `uses:` + `secrets: inherit`.
- Org-level `GITLEAKS_LICENSE` secret being inherited correctly.
- gitleaks-action uploading SARIF to the repo's Code Scanning tab.

## Run locally

```sh
cargo run
cargo test
```
