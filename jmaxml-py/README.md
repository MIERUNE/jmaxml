# jmaxml-py

## Usage

```python
from lxml import etree as et
from jmaxml import Report

with open("../assets/sample_xmls/15_12_03_161130_VPWW54.xml", "rb") as f:
    doc = et.parse(f, None)
    report = Report(doc)
    if report.mete_body:
        for warning in report.mete_body.warnings:
            for item in warning.items:
                assert item.area
                print(item.area.name)
                for kind in item.kinds:
                    print(kind.name)

```