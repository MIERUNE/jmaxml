import json
from pathlib import Path

import pandas as pd


def load_sheet(ns_prefix: str) -> pd.DataFrame:
    filenames = list(Path("assets/schema/").glob("jmaxml_*_dictionary.xls"))
    assert len(filenames) >= 1, "There are duplicate dictionary files."
    filename = filenames[0]

    return pd.read_excel(
        filename,
        sheet_name=ns_prefix,
        skiprows=3,
        dtype={
            "項番": str,
            "親要素": str,
            "子要素": str,
            "属性": str,
            "基底型": str,
            "サイズ": str,
            "出現回数": str,
            "意味": str,
            "とりうる値": str,
            "解説": str,
        },
    )


def process_prefix(ns_prefix: str) -> dict:
    sheet = load_sheet(ns_prefix)
    results = {}
    current_parent = None
    current_child = None
    for i, row in sheet.iterrows():
        if not pd.isna(row["親要素"]):
            current_parent = row["親要素"].strip()
            current_child = None
        if not pd.isna(row["子要素"]):
            current_child = row["子要素"].strip().split(":", 1)[-1]
        if not pd.isna(row["属性"]):
            current_child = "@" + row["属性"].strip().split(":", 1)[-1]

        if not pd.isna(row["とりうる値"]):
            continue

        if current_parent and current_child:
            meaning = "" if pd.isna(row["意味"]) else str(row["意味"]).strip()
            description = "" if pd.isna(row["解説"]) else str(row["解説"]).strip()

            name = f"{ns_prefix}:{current_parent}"
            results.setdefault(name, {})
            results[name][current_child] = {
                "meaning": meaning,
                "values": [],
                "description": description,
            }

    return results


def main():
    ns_prefixes = ["jmx", "jmx_ib", "jmx_eb", "jmx_mete", "jmx_seis", "jmx_volc"]
    results = {}
    for ns_prefix in ns_prefixes:
        results.update(process_prefix(ns_prefix))

    with open("assets/schema/dictionary.json", "w", encoding="utf-8") as f:
        json.dump(results, f, ensure_ascii=False, sort_keys=True, indent="\t")


if __name__ == "__main__":
    main()
