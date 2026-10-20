<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Futures modification parameter evidence

The shared Futures transport preserves each product's native request models and
validation. This record resolves the `modifyOrder` finding in
[issue #150](https://github.com/SharurTrading/binance-rs/issues/150).

| Product and transport | Pinned evidence | Client contract |
| --- | --- | --- |
| USDⓈ-M REST | `schema/usdm-rest.json`, checked 2026-09-26, requires `quantity` and `price` | Both remain required |
| USDⓈ-M WebSocket API | `schema/usdm-ws.json`, checked 2026-09-26, requires `quantity` and `price` | Both remain required |
| COIN-M REST | `schema/coinm-rest.json`, checked 2026-10-03, leaves both optional in its schema; the official endpoint text states “Either quantity or price must be sent. (After CM migration, both quantity and price are required.)” | The existing client requires both, matching the migrated contract |
| COIN-M WebSocket API | `schema/coinm-ws.json`, checked 2026-09-27, requires both; official text states “Both quantity and price must be sent.” | Both remain required |

The same pages also describe `priceMatch` and forbid combining it with `price`.
That text conflicts with the mandatory-price contract. It does not establish that
`priceMatch` replaces mandatory `price` on a modification. The client keeps the
required price and the conflict check; it does not infer an undocumented alternate
request shape. Removing the dead alternate-price guard does not relax admission.
COIN-M REST quantity-only or price-only pre-migration requests remain outside this
client's existing migrated modification contract.

`tests/futures_modify_order_contract.rs` covers omitted quantity, omitted price,
`priceMatch` without price, the valid explicit quantity/price form, and the
price/priceMatch conflict for both products and transports. Batch modifications
use this same mandatory-price contract. USDⓈ-M batch members previously accepted
quantity plus `priceMatch` without explicit price; the new regression reproduced
that acceptance before the handwritten guard was corrected.

Sources: official [USDⓈ-M REST modification](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-order),
[USDⓈ-M WebSocket API modification](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#modify-order),
[COIN-M REST modification](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-order),
and [COIN-M WebSocket API modification](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-api/trade#modify-order).
