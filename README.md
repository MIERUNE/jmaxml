# jmaxml

TODO: CI

気象庁防災情報XMLを型付きで読むためのライブラリです。今のところ Go と Python に対応しています。

気象庁が提供する XML Schema をもとにコードを生成しています。

## Usage

各言語用ライブラリのREADMEを参照してください。

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

