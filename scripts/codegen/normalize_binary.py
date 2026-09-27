# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0
"""Normalize operator-supplied official XML into offline protocol fact snapshots.

Usage: python3 scripts/codegen/normalize_binary.py XML DEST SOURCE_URL
No network, examples, prose, credentials or opaque provider implementation is copied.
"""
import hashlib
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET


def node(element):
    attrs = {key.rsplit('}', 1)[-1]: value for key, value in element.attrib.items()
             if key != 'description'}
    result = {'kind': element.tag.rsplit('}', 1)[-1], **attrs}
    if element.text and element.text.strip():
        result['value'] = element.text.strip()
    children = [node(child) for child in element]
    if children:
        result['children'] = children
    return result


def main():
    source, destination, url = sys.argv[1:]
    raw = Path(source).read_bytes()
    root = ET.fromstring(raw)
    result = {'source': url, 'sha256': hashlib.sha256(raw).hexdigest(),
              'schema': node(root)}
    Path(destination).parent.mkdir(parents=True, exist_ok=True)
    Path(destination).write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
