// 気象庁防災情報XMLで使われている要素値

package jmaxml

import (
	"encoding/xml"
	"fmt"
	"regexp"
	"strconv"
	"strings"
	"time"
)

// StringList 空白で区切られた文字列のリスト
type StringList []string

func (sl *StringList) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	s := struct {
		Value string `xml:",chardata"`
	}{}
	if err := d.DecodeElement(&s, &start); err != nil {
		return err
	}
	*sl = strings.Fields(s.Value)
	return nil
}

// NullableFloat
type NullableFloat string

func (nf NullableFloat) Value() (value float64, ok bool) {
	if nf == "" {
		return 0, false
	} else {
		v, err := strconv.ParseFloat(string(nf), 64)
		if err != nil {
			return 0, false
		}
		return v, true
	}
}

// NullableInteger
type NullableInteger string

func (nf NullableInteger) Value() (value int64, ok bool) {
	if nf == "" {
		return 0, false
	} else {
		v, err := strconv.ParseInt(string(nf), 10, 64)
		if err != nil {
			return 0, false
		}
		return v, true
	}
}

// NullableDateTime
type NullableDateTime string

func (nd NullableDateTime) Value() (value time.Time, ok bool) {
	if nd == "" {
		return time.Time{}, false
	} else {
		v, err := time.Parse(time.RFC3339Nano, string(nd))
		if err != nil {
			return time.Time{}, false
		}
		return v, true
	}
}

// Duration は ISO8601 の Duration 文字列を表わす
//
// XML Schemaの <xs:duration> に対応する
//
// Duration文字列の例
// - P1Y2M3W4DT5H6M7S
// - P1D
type Duration struct {
	Y  int `json:"y,omitempty"`  // Year
	M  int `json:"m,omitempty"`  // Month
	W  int `json:"w,omitempty"`  // Week
	D  int `json:"d,omitempty"`  // Day
	TH int `json:"th,omitempty"` // Hour
	TM int `json:"tm,omitempty"` // Minutes
	TS int `json:"ts,omitempty"` // Seconds
}

func (du *Duration) String() string {
	var sb strings.Builder
	sb.WriteString("P")
	if du.Y > 0 {
		sb.WriteString(strconv.Itoa(du.Y) + "Y")
	}
	if du.M > 0 {
		sb.WriteString(strconv.Itoa(du.M) + "M")
	}
	if du.W > 0 {
		sb.WriteString(strconv.Itoa(du.W) + "W")
	}
	if du.D > 0 {
		sb.WriteString(strconv.Itoa(du.D) + "D")
	}
	if du.TH > 0 || du.TM > 0 || du.TS > 0 {
		sb.WriteString("T")
		if du.TH > 0 {
			sb.WriteString(strconv.Itoa(du.TH) + "H")
		}
		if du.TM > 0 {
			sb.WriteString(strconv.Itoa(du.TM) + "M")
		}
		if du.TS > 0 {
			sb.WriteString(strconv.Itoa(du.TS) + "S")
		}
	}
	return sb.String()
}

func (du *Duration) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	s := struct {
		Value string `xml:",chardata"`
	}{}
	if err := d.DecodeElement(&s, &start); err != nil {
		return err
	}
	duration, err := parseISO8601Duration(s.Value)
	if err != nil {
		return err
	}
	*du = duration
	return nil
}

var durationPattern = regexp.MustCompile(`^P(\d+Y)?(\d+M)?(\d+W)?(\d+D)?(?:T(\d+H)?(\d+M)?(\d+S)?)?$`)

func parseISO8601Duration(s string) (d Duration, err error) {
	matches := durationPattern.FindStringSubmatch(s)
	if matches == nil {
		return d, fmt.Errorf("Invalid ISO8601 duration: %s", s)
	}
	for i, m := range matches[1:] {
		if m == "" {
			continue
		}
		num, _ := strconv.Atoi(m[:len(m)-1]) // エラーは起きえないので無視
		switch i {
		case 0:
			d.Y = num
		case 1:
			d.M = num
		case 2:
			d.W = num
		case 3:
			d.D = num
		case 4:
			d.TH = num
		case 5:
			d.TM = num
		case 6:
			d.TS = num
		}
	}
	return d, nil
}
