from typing import Any
from functools import cache

try:
    from lxml import etree as et

    EtreeElement = et._Element
except ImportError:
    from xml.etree import ElementTree as et


_NS = {
    "jmx": "http://xml.kishou.go.jp/jmaxml1/",
    "jmx_add": "http://xml.kishou.go.jp/jmaxml1/addition1/",
    "jmx_eb": "http://xml.kishou.go.jp/jmaxml1/elementBasis1/",
    "jmx_seis": "http://xml.kishou.go.jp/jmaxml1/body/seismology1/",
    "jmx_volc": "http://xml.kishou.go.jp/jmaxml1/body/volcanology1/",
    "jmx_mete": "http://xml.kishou.go.jp/jmaxml1/body/meteorology1/",
    "jmx_ib": "http://xml.kishou.go.jp/jmaxml1/informationBasis1/",
}


class _child:
    __slots__ = ("_coercer", "_tag", "_many")

    def __init__(self, coercer, tag: str, many: bool):
        self._coercer = coercer
        self._tag = tag
        self._many = many

    @cache
    def __get__(self, obj, _):
        if self._many:
            return [
                self._coercer(child) for child in obj._elem.iterfind(self._tag, _NS)
            ]
        else:
            if (child := obj._elem.find(self._tag, _NS)) is not None:
                return self._coercer(child)
            else:
                return None


def child(coercer, element_tag: str, many: bool = False) -> Any:
    return _child(coercer, tag=element_tag, many=many)


class _attribute:
    __slots__ = ("_coercer", "_name")

    def __init__(self, coercer, name: str):
        self._coercer = coercer
        self._name = name

    @cache
    def __get__(self, obj, _):
        if content := obj._elem.get(self._name):
            return self._coercer(content)
        return None


def attribute(coercer, name: str) -> Any:
    return _attribute(coercer, name=name)


class _text:
    __slots__ = ("_coercer",)

    def __init__(self, coercer):
        self._coercer = coercer

    @cache
    def __get__(self, obj, _):
        if (content := obj._elem.text) is not None:
            return self._coercer(content)
        return None


def text(coercer) -> Any:
    return _text(coercer)


class _childtext:
    __slots__ = ("_coercer", "_tag")

    def __init__(self, coercer, tag: str):
        self._tag = tag
        self._coercer = coercer

    @cache
    def __get__(self, obj, _):
        if (child := obj._elem.find(self._tag, _NS)) is not None:
            if t := child.text:
                return self._coercer(t)
            else:
                return None
        else:
            return None


def childtext(coercer, tag: str) -> Any:
    return _childtext(coercer, tag)


class ElementBase:
    __slots__ = ("_elem",)

    def __init__(self, _elem):
        self._elem = _elem
