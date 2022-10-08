from datetime import datetime
from glob import glob
from types import NoneType
import os.path

from jmaxml import Report
from jmaxml.custom_types import Duration
from jmaxml.utils import ElementBase
from lxml import etree as et


def check_item(item):
    if isinstance(item, NoneType):
        pass
    elif isinstance(item, list):
        for item in item:
            check_item(item)
    elif isinstance(item, ElementBase):
        traverse(item)
    elif isinstance(item, (str, int, float, datetime, Duration)):
        pass
    else:
        raise RuntimeError(f"unknown type of item: {item}")


def traverse(elem):
    keys = [key for key in dir(elem) if not key.startswith("_")]
    for key in keys:
        item = getattr(elem, key)
        check_item(item)


def test_load_all_examples():
    example_files = glob(
        os.path.join(os.path.dirname(__file__), "../../assets/sample_xmls/*.xml")
    )
    for filename in example_files:
        with open(filename, "rb") as f:
            doc = et.parse(f, None)
            report = Report(doc)
            traverse(report)
