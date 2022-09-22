# jmaxml-go

気象庁防災情報XMLを読み込むためのGo用の型定義と、いくつかのデータ型です。

Go標準パッケージの `encoding/xml` で使えます。JSON への Marshal もサポートしています。

`types_*.go` ファイルは、気象庁のXML Schemaをもとに自動生成されています。

## 使い方

次のようにしてXMLを読み込みます。

```go
import "github.com/ciscorn/jmaxml/jmaxml-go"

report := jmaxml.Report{}
if err := jmaxml.ReadReport(data, &report); err != nil {
	// Error
}
```

## カスタムのデータ型

気象庁防災情報XMLで使われている表現をサポートするため、以下の型を定義しています。

```go
// StringList は空白区切りの文字列を表す。
//
// 例: `<FooBar>Apple Grape Orange</FooBar>`
type StringList []string

// NullableDateTime は次のような omit されうるタグに対応する。NulalbleInteger, NullableFloat も同様。
//
// `<TargetDatetime>2020-03-22T00:00:00+09:00</TargetDatetime>` 
// `<TargetDatetime xsi:nil="true"/>` 
//
// 実際の値を取り出すには次のようにする。
// `value, ok := report.Control.TargetDataTime.Value()`
type NullableFloat string
type NullableInteger string
type NullableDateTime string

// Duration は ISO8601 の Duration 文字列を表す。
// XML Schemaの <xs:duration> に対応する。
//
// Duration文字列の例:
// - P1Y2M3W4DT5H6M7S
// - P1D
// - PT72H
type Duration struct {
	Year   int
	Month  int
	Week   int
	Day    int
	Hour   int
	Minute int
	Second int
}
```
