"""各言語共通のユーティリティ等"""

import json
import re

with open("./assets/schema/dictionary.json", encoding="utf-8") as f:
    dictionary = json.load(f)


def get_meaning(element_name: str, field_name: str) -> str | None:
    field_name = field_name.split(":")[-1]
    if element := dictionary.get(element_name):
        if field := element.get(field_name):
            return field.get("meaning")


def get_description(element_name: str, field_name: str) -> str | None:
    field_name = field_name.split(":")[-1]
    if element := dictionary.get(element_name):
        if field := element.get(field_name):
            meaning = field.get("meaning")
            description = field.get("description")
            if (
                description == meaning
                or description == meaning + "を示す"
                or description == meaning + "を示す。"
            ):
                return None
            return description


def pluralize(s: str) -> str:
    """名前を複数形にする"""

    if s.lower().endswith(("bearings", "values", "infos")):
        return s
    elif s.endswith("y"):
        return s[:-1] + "ies"
    elif s.endswith("is"):
        return s[:-2] + "es"
    elif (
        s.endswith("s")
        or s.endswith("x")
        or s.endswith("z")
        or s.endswith("ch")
        or s.endswith("sh")
    ):
        return s + "es"
    else:
        return s + "s"


def camel_to_snake(name) -> str:
    name = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", name).lower()
