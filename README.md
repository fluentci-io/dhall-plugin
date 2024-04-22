# Dhall Plugin

[![ci](https://github.com/fluentci-io/dhall-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/dhall-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [dhall](https://github.com/dhall-lang/dhall-lang).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm dhall setup
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of dhall.      |
| lint   | Improve Dhall code by using newer language features and removing dead code |
| freeze | Add integrity checks to remote import statements of an expression |
| format | Standard code formatter for the Dhall language |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/dhall@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: dhall
    args: |
      setup
- name: Show dhall version
  run: |
    type dhall
    dhall --version
```
