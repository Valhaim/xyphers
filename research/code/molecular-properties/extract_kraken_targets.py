#!/usr/bin/env python3
import csv
import math
import re
import sys
import xml.etree.ElementTree as ET
import zipfile


NS = "{http://schemas.openxmlformats.org/spreadsheetml/2006/main}"
SOURCE_HEADERS = {
    "A": "ID",
    "CJ": "vbur_vbur_boltz",
    "GA": "sterimol_burB5_boltz",
    "AX": "volume_boltz",
}
OUTPUT_HEADERS = ["id", "vbur_vbur_boltz", "sterimol_burB5_boltz", "volume_boltz"]


def shared_strings(archive):
    root = ET.fromstring(archive.read("xl/sharedStrings.xml"))
    return ["".join(node.text or "" for node in item.iter(NS + "t")) for item in root]


def cell_text(cell, strings):
    kind = cell.get("t")
    if kind == "inlineStr":
        return "".join(node.text or "" for node in cell.iter(NS + "t"))
    value = cell.find(NS + "v")
    if value is None or value.text is None:
        return ""
    if kind == "s":
        return strings[int(value.text)]
    return value.text


def extract(source, destination):
    with zipfile.ZipFile(source) as archive:
        strings = shared_strings(archive)
        rows = []
        with archive.open("xl/worksheets/sheet1.xml") as sheet:
            for _, row in ET.iterparse(sheet, events=("end",)):
                if row.tag != NS + "row":
                    continue
                values = {}
                for cell in row.findall(NS + "c"):
                    column = re.match(r"[A-Z]+", cell.get("r", ""))
                    if column and column.group(0) in SOURCE_HEADERS:
                        values[column.group(0)] = cell_text(cell, strings)
                rows.append(values)
                row.clear()

    headers = rows[0]
    for column, expected in SOURCE_HEADERS.items():
        if headers.get(column) != expected:
            raise ValueError(f"column {column}: expected {expected}, found {headers.get(column)!r}")

    records = []
    seen = set()
    for row_number, row in enumerate(rows[1:], start=2):
        identifier = row.get("A", "")
        if not identifier or identifier in seen:
            raise ValueError(f"missing or duplicate ID at workbook row {row_number}: {identifier!r}")
        seen.add(identifier)
        record = [identifier]
        for column in ("CJ", "GA", "AX"):
            value = float(row.get(column, ""))
            if not math.isfinite(value):
                raise ValueError(f"non-finite {column} at workbook row {row_number}")
            record.append(format(value, ".17g"))
        records.append(record)

    if len(records) != 1558:
        raise ValueError(f"expected 1558 target rows, found {len(records)}")

    with open(destination, "w", newline="", encoding="utf-8") as output:
        writer = csv.writer(output, delimiter="\t", lineterminator="\n")
        writer.writerow(OUTPUT_HEADERS)
        writer.writerows(records)
    print(f"target_rows\t{len(records)}")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit(f"usage: {sys.argv[0]} DESCRIPTORS.xlsx TARGETS.tsv")
    extract(sys.argv[1], sys.argv[2])
