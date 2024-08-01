"""各言語共通のユーティリティ等"""

import re

# 複数形化から除外する要素名の集合
_PLURALIZE_IGNORE = set(
    [
        "jmx_eb:Bearings",
        "jmx_eb:ClimateProbabilityValues",
        "jmx_mete:MeteorologicalInfos",
    ]
)


def pluralize(s: str) -> str:
    """名前を複数形にする"""

    if s in _PLURALIZE_IGNORE:
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
