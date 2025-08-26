# jmaxml

[![codecov](https://codecov.io/gh/MIERUNE/jmaxml/graph/badge.svg?token=6B5BdIgBeG)](https://codecov.io/gh/MIERUNE/jmaxml)

[気象庁防災情報XMLフォーマット](https://xml.kishou.go.jp/)の XML 電文を型付きで読むためのライブラリです。気象庁が提供する XML スキーマをもとにコード生成で作られています。読み込んだ XML を独自のルールで JSON としてシリアライズすることもでき、その JSON のための TypeScript の型宣言も提供しています。

A library for reading JMA (Japan Meteorological Agency) XML messages with type safety, created through code generation from the official XML schema. Supports JSON serialization and includes TypeScript type declarations for that JSON.

## Usage

各言語用ライブラリの README を参照してください。

- [`jmaxml-rs`](./jmaxml-rs/) - Rust 用
- [`jmaxml-wasm`](./jmaxml-wasm/) - JavaScript (WebAssembly)
    - Rust実装を利用し、XMLをパースしてJavaScriptオブジェクトに変換します。
- [`jmaxml-json-types`](./jmaxml-json-types/) - TypeScript (型宣言)
    - シリアライズされた JSON のためのの型宣言です。
- [`assets`](./assets/) - テスト用サンプルXMLとJSONを管理します。
    - `cargo run generate_samples`

おまけ（メンテナンスしていません）：

- [`jmaxml-go`](./jmaxml-go/) - Go
- [`jmaxml-py`](./jmaxml-py/) - Python

## Development

コードジェネレータは `./jmx_codegen/` ディレクトリ内で、Pythonで実装されています。

```bash
# コードジェネレータの再実行
make update
# テスト実行 (Rust, .d.ts)
make test
```

実行には [uv](https://docs.astral.sh/uv/) が必要です。

## Author

- Taku Fukada ([@ciscorn](https://github.com/ciscorn)) - Original author
- and [all contributors](https://github.com/MIERUNE/jmaxml/graphs/contributors)!
