package jmaxml_test

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/ciscorn/jmaxml/jmaxml-go"
)

// すべてのサンプル電文について読み込みが成功するか確認する
func TestReadReport(t *testing.T) {
	metches, err := filepath.Glob("../assets/sample_xmls/*.xml")
	if err != nil {
		t.Fatalf("failed to glob sample XMLs: %v", err)
	}
	if len(metches) == 0 {
		t.Fatalf("no XML samples found")
	}

	for _, path := range metches {
		data, err := os.ReadFile(path)
		if err != nil {
			t.Fatalf("failed to open sample XML: %s", path)
		}
		report := jmaxml.Report{}
		if err := jmaxml.ReadReport(data, &report); err != nil {
			t.Errorf("failed to read JMA XML: %s", path)
		}

		// 基本的な要素のチェック
		if report.Control.Title == "" {
			t.Errorf("Control > Title is empty: %s", path)
		}
		status := report.Control.Status
		if status != "通常" && status != "訓練" && status != "試験" {
			t.Errorf("Status is invalid: %s", path)
		}
		if len(report.Control.PublishingOffices) == 0 {
			t.Errorf("PublishingOffices is empty: %s", path)
		}
		if report.Control.EditorialOffice == "" {
			t.Errorf("EditorialOffice is empty: %s", path)
		}
		if report.Head.InfoType == "" {
			t.Errorf("InfoType is empty: %s", path)
		}
		if report.Head.InfoKind == "" {
			t.Errorf("InfoKind is empty: %s", path)
		}
		if report.Head.InfoKindVersion == "" {
			t.Errorf("InfoKindVersion is empty: %s", path)
		}
		if report.Head.Title == "" {
			t.Errorf("Head > Title is empty: %s", path)
		}

		// 各種Bodyのうちどれか1つだけが存在することを確認する
		bodyCount := 0
		if report.SeisBody != nil {
			bodyCount += 1
		}
		if report.MeteBody != nil {
			bodyCount += 1
		}
		if report.VolcBody != nil {
			bodyCount += 1
		}
		if bodyCount != 1 {
			t.Errorf("There must be only 1 body but found %d bodies", bodyCount)
		}
	}
}
