"""気象庁防災情報XMLの XML Schema を読み込んで、中間表現を作る"""

import os.path
import pathlib
from typing import Dict, cast

import lxml.etree as et

from .types import (
    XsAttribute,
    XsBase,
    XsChildElement,
    XsComplexType,
    XsElement,
    XsEnumeration,
    XsPrimitive,
    XsSchema,
    XsTypeName,
)

_NS = {
    "xs": "http://www.w3.org/2001/XMLSchema",
}

_JMX_BODIES = ["jmx_mete:Body", "jmx_seis:Body", "jmx_volc:Body"]


def _parse_simple_type(
    type_map: Dict[XsTypeName, XsBase], prefix: str, simple_type
) -> None:
    """
    xs:simpleType をParseする
    """
    assert len(simple_type) == 1
    name = f"{prefix}:{simple_type.get('name')}"

    if (restriction := simple_type.find("./xs:restriction", _NS)) is not None:
        _type = restriction.get("base")
        assert _type == "xs:string" or _type == "xs:token"
        values = [e.get("value") for e in restriction.iterfind("./xs:enumeration", _NS)]
        type_map[name] = XsEnumeration(name=name, type=_type, values=values)

    elif (union := simple_type.find("./xs:union", _NS)) is not None:
        members = union.get("memberTypes").split()
        members = [m for m in members]
        types = set(cast(XsPrimitive, type_map[member]).type for member in members)
        assert len(types) == 1
        type_map[name] = type_map[types.pop()]

    elif (_list := simple_type.find("./xs:list", _NS)) is not None:
        assert _list.get("itemType") == "xs:string"
        type_map[name] = XsPrimitive("StringList", "StringList")

    else:
        raise RuntimeError("Unknown simple type found")


def _parse_complex_type(
    type_map: Dict[XsTypeName, XsBase], prefix: str, complex_type
) -> None:
    """
    xs:complexType をParseする
    """

    simple_content = complex_type.find("./xs:simpleContent", _NS)
    sequences = complex_type.findall("./xs:sequence", _NS)

    # complexTypeの内容が <xs:simpleContent> の場合
    if simple_content is not None:
        assert len(simple_content) == 1
        extension = simple_content.find("./xs:extension", _NS)
        content_type = extension.get("base")
        attribute_elems = extension.findall("./xs:attribute", _NS)
        attributes = {}
        for attr in attribute_elems:
            name: str = attr.get("name")
            attributes[name] = XsAttribute(
                name=name, type=attr.get("type"), use=attr.get("use")
            )

        name = f"{prefix}:{complex_type.get('name')}"
        type_map[name] = XsComplexType(
            name=name, content_type=content_type, attributes=attributes, elements=[]
        )

    # complexTypeの内容が <xs:sequence> の場合
    elif sequences:
        assert len(sequences) == 1
        sequence = sequences[0]

        # 子要素を処理する
        children = []
        for element in sequence.iterfind("./xs:element", _NS):
            maxOccurs = element.get("maxOccurs")
            if maxOccurs is not None:
                if maxOccurs == "unbounded":
                    maxOccurs = None
                else:
                    maxOccurs = int(maxOccurs)
            minOccurs = element.get("minOccurs")
            if minOccurs is not None:
                minOccurs = int(minOccurs)

            if _type := element.get("type"):
                name = f"{prefix}:{element.get('name')}"

                if element.get("nillable") == "true":
                    # <xs:element ... name="TargetDateTime" nillable="true" .../> への対応
                    _type = _type + "-nillable"

                child = XsChildElement(
                    name=name,
                    type=_type,
                    ref=None,
                    max_occurs=maxOccurs,
                    min_occurs=minOccurs,
                )
                children.append(child)
            elif ref := element.get("ref"):
                child = XsChildElement(
                    name=None,
                    type=None,
                    ref=ref,
                    max_occurs=maxOccurs,
                    min_occurs=minOccurs,
                )
                children.append(child)

        # 属性値を処理する
        attribute_elems = complex_type.findall("./xs:attribute", _NS)
        attributes = {}
        for attr in attribute_elems:
            name = f"{prefix}:{attr.get('name')}"
            attributes[name] = XsAttribute(
                name=name, type=attr.get("type"), use=attr.get("use")
            )

        # <xs:any> を処理する
        name = f"{prefix}:{complex_type.get('name')}"
        if any := sequence.findall("./xs:any", _NS):
            assert len(any) == 1
            any_namespace = any[0].get("namespace")
            if any_namespace == "##other":
                # namespace="##other" は、jmx:Report要素に各種のBody要素を含めるために使われている
                assert prefix == "jmx"
                for body_ref in _JMX_BODIES:
                    children.append(
                        XsChildElement(
                            name=None,
                            type=None,
                            ref=body_ref,
                            max_occurs=1,
                            min_occurs=0,
                        )
                    )
            elif any_namespace == "http://xml.kishou.go.jp/jmaxml1/addition1/":
                # Schema外の任意の要素を含める addition については、
                # 今のところ使わないため対応しない
                pass
            else:
                raise RuntimeError(f"Unknown xs:any namespace: {any_namespace}")

        type_map[name] = XsComplexType(
            name=name,
            elements=children,
            attributes=attributes,
            content_type=None,
        )


def _parse_doc(type_map: Dict[XsTypeName, XsBase], prefix: str, doc):
    """
    気象庁電文 の XML Schema をParseする
    """
    for element in doc.iterfind("./xs:element", _NS):
        name = prefix + ":" + element.get("name")
        _type = element.get("type")
        type_map[name] = XsElement(name=name, type=_type)

    for simple_type in doc.iterfind("./xs:simpleType", _NS):
        _parse_simple_type(type_map, prefix, simple_type)

    for complex_type in doc.iterfind("./xs:complexType", _NS):
        _parse_complex_type(type_map, prefix, complex_type)


def _validate_type_map(type_map):
    """
    各typeが参照するtypeが存在していることをチェックする
    """
    for value in type_map.values():
        if isinstance(value, (XsAttribute, XsElement)):
            assert value.type in type_map

        if isinstance(value, XsComplexType):
            for elem in value.elements:
                if elem.ref:
                    assert elem.ref in type_map
                else:
                    assert elem.type in type_map, elem.type

        if isinstance(value, XsComplexType):
            if value.content_type:
                assert value.content_type in type_map, value.content_type
            for attr in value.attributes.values():
                assert attr.type in type_map, attr


def load_schema(xsd_dir: str) -> XsSchema:
    type_map: Dict[XsTypeName, XsBase] = {}
    primitives = [
        "xs:dateTime-nillable",
        "jmx_eb:nullablefloat",
        "jmx_eb:nullableinteger",
        "xs:anyURI",
        "xs:dateTime",
        "xs:duration",
        "xs:float",
        "xs:gMonthDay",
        "xs:int",
        "xs:integer",
        "xs:string",
        "xs:token",
        "xs:boolean",
        "xs:unsignedByte",
    ]
    for prim in primitives:
        type_map[prim] = XsPrimitive(prim, prim)

    for filename in pathlib.Path(xsd_dir).glob("*.xsd"):
        with open(filename) as f:
            doc = et.parse(f, None)
            xml_prefix = os.path.basename(filename).split(".")[0]
            _parse_doc(type_map, xml_prefix, doc)

    _validate_type_map(type_map)

    return XsSchema(type_map=type_map)
