# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Withdrawal routes must never return to the generated executable catalog."""
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]


class WalletScope(unittest.TestCase):
    def test_external_withdrawals_are_excluded(self):
        coverage = json.loads((ROOT / 'schema/wallet-coverage.json').read_text())
        forbidden = {
            '/sapi/v1/capital/withdraw/apply',
            '/sapi/v1/localentity/withdraw/apply',
            '/sapi/v1/localentity/broker/withdraw/apply',
            '/sapi/v1/account/enableFastWithdrawSwitch',
            '/sapi/v1/account/disableFastWithdrawSwitch',
            '/sapi/v1/capital/withdraw/address/list',
            '/sapi/v1/addressVerify/list',
        }
        self.assertFalse(forbidden & {op['path'] for op in coverage['rest']})
        self.assertEqual(forbidden, {op['path'] for op in coverage['excluded']})
        self.assertTrue(all(op['reason'] for op in coverage['excluded']))


if __name__ == '__main__':
    unittest.main()
