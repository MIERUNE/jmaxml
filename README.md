# jmaxml

Work in Progress

[気象庁防災情報XMLフォーマット](https://xml.kishou.go.jp/)のXML電文を型付きで読むためのライブラリです。気象庁が提供する XML Schema をもとにコード生成で作られています。

## Usage

各言語用ライブラリのREADMEを参照してください。

- [Rust](./jmaxml-rs/)
- [Go](./jmaxml-go/)
- [Python](./jmaxml-py/)

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
