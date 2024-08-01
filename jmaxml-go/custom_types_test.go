package jmaxml_test

import (
	"encoding/xml"
	"fmt"
	"math"
	"testing"

	"github.com/MIERUNE/jmaxml/jmaxml-go"
)

func TestStringList(t *testing.T) {
	blob := `<StringList> aaa bbb ccc </StringList>`
	var sl jmaxml.StringList
	if err := xml.Unmarshal([]byte(blob), &sl); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	if len(sl) != 3 && (sl[0] != "aaa" || sl[1] != "bbb" || sl[2] == "ccc") {
		t.Errorf("result must be [aaa bbb ccc]")
	}

	// error case
	blob = `<StringList> < </StringList>`
	if err := xml.Unmarshal([]byte(blob), &sl); err == nil {
		t.Errorf("should fail")
	}
}

func TestNullableDateTime(t *testing.T) {
	blob := `<Dummy />`
	var nd jmaxml.NullableDateTime
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	if _, ok := nd.Value(); ok {
		t.Errorf("value must be null")
	}

	blob = `<Dummy></Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	if _, ok := nd.Value(); ok {
		t.Errorf("value must be null")
	}

	blob = `<Dummy>2019-07-03T14:47:24+09:00</Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	value, ok := nd.Value()
	if !ok {
		t.Errorf("value must be null")
	}
	if value.IsZero() {
		t.Errorf("invalid date")
	}

	blob = `<Dummy>2019-07-03T1:7:4+0900</Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	value, ok = nd.Value()
	if ok {
		t.Errorf("must be not ok")
	}
	if !value.IsZero() {
		t.Errorf("must be parsed as valid date")
	}
}

func TestNullableInteger(t *testing.T) {
	blob := `<Dummy />`
	var nd jmaxml.NullableInteger
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	if _, ok := nd.Value(); ok {
		t.Errorf("value must be null")
	}
	blob = `<Dummy>123</Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	v, ok := nd.Value()
	if !ok || v != 123 {
		t.Errorf("value must be 123")
	}

	// error case
	blob = `<Dummy>abc</Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	v, ok = nd.Value()
	if ok || v != 0 {
		t.Errorf("must raise error")
	}
}

func TestNullableFloat(t *testing.T) {
	blob := `<Dummy />`
	var nd jmaxml.NullableFloat
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	if _, ok := nd.Value(); ok {
		t.Errorf("value must be null")
	}
	blob = `<Dummy>1234.5678</Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	v, ok := nd.Value()
	if !ok || math.Abs(v-1234.5678) > 1e-10 {
		t.Errorf("value must be 1234.5678")
	}

	// error case
	blob = `<Dummy>abc</Dummy>`
	if err := xml.Unmarshal([]byte(blob), &nd); err != nil {
		t.Errorf("failed to unmarshal: %s", err)
	}
	v, ok = nd.Value()
	if ok || v != 0 {
		t.Errorf("must raise error")
	}
}

func TestDuration(t *testing.T) {
	testcases := []struct {
		Repl     string
		Invalid  bool
		Expected jmaxml.Duration
	}{
		{
			Repl: "PT55S",
			Expected: jmaxml.Duration{
				TS: 55,
			},
		},
		{
			Repl: "P7M",
			Expected: jmaxml.Duration{
				M: 7,
			},
		},
		{
			Repl: "P3YT10S",
			Expected: jmaxml.Duration{
				Y:  3,
				TS: 10,
			},
		},
		{
			Repl: "P1DT72H",
			Expected: jmaxml.Duration{
				D:  1,
				TH: 72,
			},
		},
		{
			Repl: "P12WT72M",
			Expected: jmaxml.Duration{
				W:  12,
				TM: 72,
			},
		},
		{
			Repl: "P33D",
			Expected: jmaxml.Duration{
				D: 33,
			},
		},
		{
			Repl:    "1DT72H",
			Invalid: true,
		},
		{
			Repl:    "P10S",
			Invalid: true,
		},
		{
			Repl:    "PXS",
			Invalid: true,
		},
		{
			Repl:    "<",
			Invalid: true,
		},
	}

	for _, testcase := range testcases {
		blob := fmt.Sprintf(`<DateTime>%s</DateTime>`, testcase.Repl)
		var d jmaxml.Duration
		if err := xml.Unmarshal([]byte(blob), &d); (err != nil) != testcase.Invalid {
			t.Errorf("invalid: %v, err: %v", testcase.Invalid, err)
			continue
		}
		if testcase.Invalid {
			continue
		}
		if d != testcase.Expected {
			t.Errorf("expected result is %+v, but got %+v", testcase.Expected, d)
		}
		if s := d.String(); s != testcase.Repl {
			t.Errorf("expected %+v, but got %+v", testcase.Repl, s)
		}
	}
}
