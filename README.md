# jmaxml

Work in Progress

気象庁防災情報XMLを型付きで読むためのライブラリです。気象庁が提供する XML Schema をもとにコード生成で作られています。

## Usage

各言語用ライブラリのREADMEを参照してください。

- [Rust](./jmaxml-go/)
- [Go](./jmaxml-go/)
- [Python](./jmaxml-py/)
- [TypeScript](./jmaxml-ts/)

## Development

コードジェネレータは `./jmx_codegen/` ディレクトリ内で実装されています。

```bash
# コードジェネレータの再実行
make update

# テストの実行
make test
```

