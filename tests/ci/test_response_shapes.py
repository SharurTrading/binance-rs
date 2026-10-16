# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Adding an exposed JSON method requires independent response evidence."""
import json
import hashlib
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]


class ResponseShapes(unittest.TestCase):
    def test_every_executable_method_has_response_evidence(self):
        corpus = json.loads((ROOT / 'tests/fixtures/response-shapes.json').read_text())
        expected = set()
        for path in (ROOT / 'schema').glob('*.json'):
            product, _, transport = path.stem.rpartition('-')
            if transport not in {'rest', 'ws'}:
                continue
            snapshot = json.loads(path.read_text())
            for operation in snapshot['operations']:
                if operation.get('x-excluded-reason'):
                    continue
                expected.add((product.replace('-', '_'), transport, operation['operationId']))
        for product in ('spot', 'usdm', 'coinm'):
            for operation in ('session.logon', 'session.status', 'session.logout'):
                expected.add((product, 'session', operation))
        expected.add(('margin', 'ws', 'userDataStream.subscribe.listenToken'))
        actual = {(op['product'], op['transport'], op['operation'])
                  for op in corpus['operations']}
        self.assertEqual(expected, actual)
        cases = {(case['product'], case['transport'], case['operation'])
                 for case in corpus['cases']}
        self.assertEqual(actual, cases)
        self.assertEqual(len(corpus['cases']), len({c['id'] for c in corpus['cases']}))
        self.assertTrue(all(c['source'].startswith('https://') for c in corpus['cases']))

    def test_every_stream_route_and_private_dto_has_evidence(self):
        corpus = json.loads((ROOT / 'tests/fixtures/stream-shapes.json').read_text())
        actual = {(case['product'], case['operation']) for case in corpus['cases']}
        for path in (ROOT / 'schema').glob('*-streams.json'):
            product = path.stem[:-8].replace('-', '_')
            snapshot = json.loads(path.read_text())
            expected = {(product, op['operationId']) for op in snapshot['operations']}
            self.assertFalse(expected - actual)
        # Public user-event alternatives are another inventory, outside route snapshots.
        import re
        for path in (ROOT / 'src').glob('*_trading/*/event_payloads.rs'):
            text = path.read_text()
            if 'pub enum UserPayload {' not in text:
                continue
            enum = text.split('pub enum UserPayload {', 1)[1].split('\n}', 1)[0]
            models = ' '.join(case['model'] for case in corpus['cases']
                              if case['product'] == path.parent.name)
            for model in re.findall(r'stream_models::(\w+)', enum):
                self.assertIn(model, models, (path, model))

    def test_every_pinned_binary_definition_has_wire_evidence(self):
        corpus = json.loads((ROOT / 'tests/fixtures/binary-shapes.json').read_text())
        for source in corpus['sources']:
            if 'snapshot' not in source:
                continue
            snapshot = json.loads((ROOT / source['snapshot']).read_text())
            self.assertEqual(snapshot['sha256'], source['sha256'])
            nodes = snapshot['schema']['children']
            if source['protocol'] in {'oe', 'md'}:
                messages = next(n for n in nodes if n['kind'] == 'messages')['children']
                expected = {n['msgtype'] for n in messages}
                actual = {c['message_type'] for c in corpus['cases']
                          if c['source'] == source['id']}
            else:
                expected = {int(n['id']) for n in nodes if n['kind'] == 'message'}
                actual = {c['template_id'] for c in corpus['cases']
                          if c['source'] == source['id']}
            self.assertEqual(expected, actual)
        for case in corpus['cases']:
            wire = bytes.fromhex(case['hex'])
            self.assertEqual(len(wire), case['length'])
            self.assertEqual(hashlib.sha256(wire).hexdigest(), case['wire_sha256'])


if __name__ == '__main__':
    unittest.main()
