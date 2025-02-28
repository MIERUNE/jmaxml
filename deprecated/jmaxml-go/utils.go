package jmaxml

import "encoding/xml"

func ReadReport(data []byte, report *Report) error {
	err := xml.Unmarshal(data, report)
	return err
}
