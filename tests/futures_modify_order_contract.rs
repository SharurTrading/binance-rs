// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native modification requiredness survives shared transport extraction.
//! See docs/futures-modify-order-evidence.md for the contradictory priceMatch text.

use binance_client::{Decimal, Error, Symbol, coinm, usdm};

macro_rules! modification_contract {
    ($name:ident, $request:path, $symbol:literal) => {
        #[test]
        fn $name() -> Result<(), Error> {
            let base = <$request>::new()
                .symbol(Symbol::new($symbol)?)
                .side("BUY")
                .order_id(17);
            assert!(base.clone().price(Decimal::from(100)).build().is_err());
            assert!(base.clone().quantity(Decimal::ONE).build().is_err());
            assert!(
                base.clone()
                    .quantity(Decimal::ONE)
                    .price_match("OPPONENT")
                    .build()
                    .is_err()
            );
            assert!(
                base.clone()
                    .quantity(Decimal::ONE)
                    .price(Decimal::from(100))
                    .build()
                    .is_ok()
            );
            assert!(
                base.quantity(Decimal::ONE)
                    .price(Decimal::from(100))
                    .price_match("OPPONENT")
                    .build()
                    .is_err()
            );
            Ok(())
        }
    };
}

modification_contract!(
    linear_rest_requires_quantity_and_price,
    usdm::rest_requests::ModifyOrder,
    "BTCUSDT"
);
modification_contract!(
    linear_ws_requires_quantity_and_price,
    usdm::ws_requests::ModifyOrder,
    "BTCUSDT"
);
modification_contract!(
    inverse_rest_preserves_migrated_quantity_and_price,
    coinm::rest_requests::ModifyOrder,
    "BTCUSD_PERP"
);
modification_contract!(
    inverse_ws_requires_quantity_and_price,
    coinm::ws_requests::ModifyOrder,
    "BTCUSD_PERP"
);

#[test]
fn linear_batch_modification_requires_the_documented_explicit_price() -> Result<(), Error> {
    let member = usdm::rest_models::ModifyMultipleOrdersBatchOrdersInputItem::new()
        .symbol(Symbol::new("BTCUSDT")?)
        .side("BUY")
        .order_id(17)
        .quantity(Decimal::ONE);
    assert!(member.clone().price_match("OPPONENT").build().is_err());
    assert!(
        member
            .clone()
            .price(Decimal::from(100))
            .price_match("OPPONENT")
            .build()
            .is_err()
    );
    let valid_member = member.price(Decimal::from(100)).build()?;
    assert!(
        usdm::rest_requests::ModifyMultipleOrders::new()
            .batch_orders(vec![valid_member])
            .build()
            .is_ok()
    );
    Ok(())
}
