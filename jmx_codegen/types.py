"""気象庁の XML Schema を表現するための一連の型です"""

from dataclasses import dataclass
from typing import Dict, Optional, Sequence

XsTypeName = str


@dataclass
class XsChildElement:
    name: Optional[str]
    type: Optional[XsTypeName]
    ref: Optional[str]
    max_occurs: Optional[int]
    min_occurs: Optional[int]


@dataclass
class XsBase:
    name: str


@dataclass
class XsAttribute(XsBase):
    use: Optional[str]
    type: XsTypeName


@dataclass
class XsComplexType(XsBase):
    content_type: Optional[XsTypeName]
    elements: Sequence[XsChildElement]
    attributes: Dict[str, XsAttribute]


@dataclass
class XsPrimitive(XsBase):
    type: XsTypeName


@dataclass
class XsEnumeration(XsBase):
    type: XsTypeName
    values: list[str]


@dataclass
class XsElement(XsBase):
    type: XsTypeName


@dataclass
class XsSchema:
    type_map: Dict[XsTypeName, XsBase]
