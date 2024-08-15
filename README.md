# jmaxml

Work in Progress

[気象庁防災情報XMLフォーマット](https://xml.kishou.go.jp/)のXML電文を型付きで読むためのライブラリです。気象庁が提供する XML Schema をもとにコード生成で作られています。

## Usage

各言語用ライブラリのREADMEを参照してください。

- [`jmaxml-rs`](./jmaxml-rs/) - Rust
    - [`jmaxml-wasm`](./jmaxml-wasm/) - JavaScript (Wasm)
- [`jmaxml-go`](./jmaxml-go/) - Go
- [`jmaxml-py`](./jmaxml-py/) - Python

## Development

コードジェネレータは `./jmx_codegen/` ディレクトリ内で実装されています。

```bash
# コードジェネレータの再実行
make update

# テストの実行
make test
```

## Authors

- MIERUNE Inc.
- Taku Fukada (original author)
