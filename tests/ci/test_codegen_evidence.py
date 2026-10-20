# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Structural fixture assertions must retain nested unknown-evidence detection."""
import importlib.util
from pathlib import Path
import tempfile
import sys
from types import SimpleNamespace
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('response_shapes', ROOT/'scripts/codegen/response_shapes.py')
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class CodegenEvidence(unittest.TestCase):
    def test_provider_description_formats_identifiers_without_changing_markdown(self):
        sys.path.insert(0, str(ROOT/'scripts/codegen'))
        try:
            import generate
        finally:
            sys.path.pop(0)
        text = 'BNBBurn, marginLevel and MARGIN_CALL; `orderId` [wire](https://example.test/orderId).'
        rendered = generate.description_docs(text)
        self.assertEqual(rendered, '`BNBBurn`, `marginLevel` and `MARGIN_CALL`; `orderId` [wire](https://example.test/orderId).')

    def test_repeated_aliases_and_multiline_nested_types_keep_unknown_checks(self):
        source = '''pub enum Member {
    Known(String),
    Unknown(super::event_payloads::UnknownMessage),
}
pub type Receipt = Member;
pub struct Response {
    pub nested: Option<
        Vec<Receipt>
    >,
}
pub enum Stream {
    Rows(
        Box<Response>,
    ),
    Unknown(super::event_payloads::UnknownMessage),
}
'''
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root/'src').mkdir()
            (root/'src/models.rs').write_text(source)
            checks = MODULE.EvidenceChecks(SimpleNamespace(ROOT=root))
            first = checks.predicate('binance_client::models::Receipt')
            second = checks.predicate('binance_client::models::Receipt')
            self.assertEqual(first, second)
            self.assertNotEqual(first, '|_|false')
            self.assertNotEqual(checks.predicate('binance_client::models::Response'), '|_|false')
            self.assertNotEqual(checks.predicate('binance_client::models::Stream'), '|_|false')
            code = checks.code()
            self.assertIn('.iter().any(', code)
            self.assertIn('.as_ref().is_some_and(', code)
            self.assertIn('binance_client::models::Stream::Rows(item)', code)
            self.assertIn('binance_client::models::Member::Unknown(_)', code)


if __name__ == '__main__':
    unittest.main()
