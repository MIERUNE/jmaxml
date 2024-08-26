# jmaxml

Work in Progress

[気象庁防災情報XMLフォーマット](https://xml.kishou.go.jp/)の XML 電文を型付きで読むためのライブラリです。気象庁が提供する XML スキーマをもとにコード生成で作られています。

読み込んだ XML を独自のルールで JSON としてシリアライズすることもできます。シリアライズされた JSON のための TypeScript の型宣言も用意しています。

## Usage

各言語用ライブラリの README を参照してください。

- [`jmaxml-rs`](./jmaxml-rs/) - Rust 用
- [`jmaxml-wasm`](./jmaxml-wasm/) - JavaScript (WebAssembly)
    - Rust実装を利用し、XMLをパースしてJavaScriptオブジェクトに変換します。
- [`jmaxml-json-types`](./jmaxml-json-types/) - TypeScript (型宣言)
    - シリアライズされた JSON のためのの型宣言です。

おまけ（メンテナンスしていません）：

- [`jmaxml-go`](./jmaxml-go/) - Go
- [`jmaxml-py`](./jmaxml-py/) - Python

## Development

コードジェネレータは `./jmx_codegen/` ディレクトリ内で実装されています。

```bash
# コードジェネレータの再実行
make update
```

## Authors

- MIERUNE Inc.
- Taku Fukada (original author)

