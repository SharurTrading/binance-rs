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


def load_generate():
    sys.path.insert(0, str(ROOT/'scripts/codegen'))
    try:
        import generate
    finally:
        sys.path.pop(0)
    return generate


def shared_filter(setter_doc, field_type='string'):
    return {'type': 'object', 'x-rust-type-name': 'SharedFilterInput', 'x-setter-doc': setter_doc,
            'x-evidence': [{'source': setter_doc}],
            'properties': {'side': {'type': field_type, 'description': setter_doc}}}


class CodegenEvidence(unittest.TestCase):
    def test_provider_description_formats_identifiers_without_changing_markdown(self):
        generate = load_generate()
        text = 'BNBBurn, marginLevel and MARGIN_CALL; `orderId` [wire](https://example.test/orderId).'
        rendered = generate.description_docs(text)
        self.assertEqual(rendered, '`BNBBurn`, `marginLevel` and `MARGIN_CALL`; `orderId` [wire](https://example.test/orderId).')

    def test_setter_doc_replaces_stock_setter_line_and_keeps_wire_key(self):
        generate = load_generate()
        rendered = generate.setter_summary('filter', {'x-setter-doc': 'Matches executionReport only.'}, 'optional')
        self.assertEqual(rendered, 'Matches `executionReport` only.\n    /// Wire key: `filter`; optional.')
        stock = generate.setter_summary('filter', {}, 'required')
        self.assertEqual(stock, 'Set the provider `filter` parameter (required).')

    def test_shared_request_type_name_ignores_documentation_differences(self):
        generate = load_generate()
        models = generate.Models({})
        first = models.type(shared_filter('First caveat.'), 'FirstFilter', response=False)
        second = models.type(shared_filter('Second caveat.'), 'SecondFilter', response=False)
        self.assertEqual((first, second), ('SharedFilterInput', 'SharedFilterInput'))

    def test_shared_request_type_name_with_different_properties_is_refused(self):
        generate = load_generate()
        models = generate.Models({})
        models.type(shared_filter('Caveat.'), 'FirstFilter', response=False)
        with self.assertRaisesRegex(ValueError, 'shared request type name with different properties'):
            models.type(shared_filter('Caveat.', field_type='integer'), 'SecondFilter', response=False)

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
