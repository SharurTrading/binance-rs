// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Financial precision, request refusal, and depth continuity contracts.

#[cfg(test)]
mod tests {
    use binance_client::usdm::{
        book::{BookState, DepthBook},
        rest_models::{OrderBookResponse, PlaceMultipleOrdersBatchOrdersInputItem},
        rest_requests::{NewOrder, PlaceMultipleOrders},
        stream_models::{DiffBookDepthStreamsEvent, OrderTradeUpdateEvent},
        wire::PriceLevel,
    };
    use binance_client::{ClientOrderId, Decimal, Symbol};
    use serde_json::json;

    fn dec(value: &str) -> Decimal {
        Decimal::from_str_exact(value).unwrap()
    }
    fn order() -> NewOrder {
        NewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(dec("0.001"))
            .new_client_order_id(ClientOrderId::new("fixture-1").unwrap())
    }
    fn update(first: i64, last: i64, previous: i64, quantity: &str) -> DiffBookDepthStreamsEvent {
        serde_json::from_value(json!({"e":"depthUpdate","E":1,"T":1,"s":"BTCUSDT","U":first,"u":last,"pu":previous,"b":[["100.00000000000000000001",quantity]],"a":[]})).unwrap()
    }
    fn snapshot() -> OrderBookResponse {
        serde_json::from_value(
        json!({"lastUpdateId":100,"bids":[["100.00000000000000000001","1"]],"asks":[["101","2"]]}),
    )
    .unwrap()
    }

    #[test]
    fn money_is_exact_and_unrepresentable_values_are_refused() {
        let level: PriceLevel =
            serde_json::from_str("[100.00000000000000000001,\"0.0000000000000000000000000001\"]")
                .unwrap();
        assert_eq!(level.price, dec("100.00000000000000000001"));
        assert_eq!(level.quantity, dec("0.0000000000000000000000000001"));
        for bad in [
            "[\"1\",\"1e-29\"]",
            "[\"NaN\",\"1\"]",
            "[\"1\",\"-1\"]",
            "[\"1\",\"1\",\"extra\"]",
        ] {
            assert!(serde_json::from_str::<PriceLevel>(bad).is_err());
        }
    }

    #[test]
    fn invalid_single_and_nested_orders_are_refused() {
        assert!(order().build().is_ok());
        assert!(order().quantity(Decimal::ZERO).build().is_err());
        assert!(order().side("future-unknown-side").build().is_err());
        assert!(
            order()
                .position_side("LONG")
                .reduce_only("false")
                .build()
                .is_err()
        );
        let nested = PlaceMultipleOrdersBatchOrdersInputItem::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(dec("-1"))
            .new_client_order_id(ClientOrderId::new("nested-1").unwrap());
        assert!(
            PlaceMultipleOrders::new()
                .batch_orders(vec![nested])
                .build()
                .is_err()
        );
    }

    #[test]
    fn malformed_known_execution_data_cannot_be_a_success() {
        assert!(
            serde_json::from_value::<OrderTradeUpdateEvent>(json!({"e":"ORDER_TRADE_UPDATE"}))
                .is_err()
        );
        assert!(
            serde_json::from_value::<OrderTradeUpdateEvent>(
                json!({"e":"ORDER_TRADE_UPDATE","E":1,"T":1,"o":{}})
            )
            .is_err()
        );
    }

    #[test]
    fn snapshot_requires_a_bridge_and_deletion_is_absolute() {
        let mut book = DepthBook::new(Symbol::new("BTCUSDT").unwrap(), 7);
        book.snapshot(&snapshot()).unwrap();
        assert_eq!(book.state(), BookState::AwaitingSnapshot);
        assert!(book.bids().is_err());
        book.update(7, update(99, 101, 98, "3")).unwrap();
        assert_eq!(
            book.bids().unwrap()[&dec("100.00000000000000000001")],
            dec("3")
        );
        book.update(7, update(102, 102, 101, "0")).unwrap();
        assert!(book.bids().unwrap().is_empty());
        assert!(book.is_partial());
        assert!(book.update(7, update(104, 104, 103, "4")).is_err());
        assert_eq!(book.state(), BookState::Gap);
        assert!(book.bids().is_err());
    }

    #[test]
    fn bootstrap_retains_a_large_source_ordered_prefix() {
        let mut book = DepthBook::new(Symbol::new("BTCUSDT").unwrap(), 7);
        for id in 100..5100 {
            book.update(7, update(id, id, id - 1, &id.to_string()))
                .unwrap();
        }
        book.snapshot(&snapshot()).unwrap();
        assert_eq!(book.last_update_id(), Some(5099));
        assert_eq!(
            book.bids().unwrap()[&dec("100.00000000000000000001")],
            Decimal::from(5099)
        );
        assert!(book.update(8, update(5100, 5100, 5099, "1")).is_err());
    }

    #[test]
    fn bootstrap_gap_reports_the_count_of_discarded_pending_updates() {
        let mut book = DepthBook::new(Symbol::new("BTCUSDT").unwrap(), 7);
        assert_eq!(book.discarded_pending_updates(), 0);
        // The first buffered update bridges, the second breaks the `pu` chain
        // mid-drain, and the third is never applied: two updates are discarded.
        book.update(7, update(99, 101, 98, "1")).unwrap();
        book.update(7, update(103, 103, 102, "2")).unwrap();
        book.update(7, update(104, 104, 103, "3")).unwrap();
        assert!(book.snapshot(&snapshot()).is_err());
        assert_eq!(book.state(), BookState::Gap);
        assert_eq!(book.discarded_pending_updates(), 2);
        assert!(book.bids().is_err());
        assert!(book.asks().is_err());
        // A later snapshot that bridges the surviving evidence restores views
        // and clears the discarded count.
        let recovery: OrderBookResponse = serde_json::from_value(
            json!({"lastUpdateId":104,"bids":[["100.00000000000000000001","1"]],"asks":[]}),
        )
        .unwrap();
        book.snapshot(&recovery).unwrap();
        assert_eq!(book.state(), BookState::Ready);
        assert_eq!(book.discarded_pending_updates(), 0);
        assert_eq!(book.last_update_id(), Some(104));
    }

    #[test]
    fn crossed_funding_update_is_partial_and_does_not_fabricate_positions() {
        let event:binance_client::usdm::stream_models::AccountUpdateEvent=serde_json::from_value(json!({"e":"ACCOUNT_UPDATE","E":1,"T":1,"a":{"m":"FUNDING_FEE","B":[{"a":"USDT","wb":"10.1","cw":"10.1","bc":"-0.1"}]}})).unwrap();
        assert!(event.a.upper_p.is_none());
        assert_eq!(event.a.upper_b.unwrap()[0].bc, dec("-0.1"));
    }

    #[test]
    fn scientific_mantissas_never_silently_round_and_klines_roundtrip_as_columns() {
        assert!(
            serde_json::from_str::<PriceLevel>("[\"1.123456789012345678901234567890123e1\",\"1\"]")
                .is_err()
        );
        let columns = json!([
            1, "1.1", "1.2", "1.0", "1.15", "5.5", 2, "6.2", 3, "2.1", "2.3", "0"
        ]);
        let kline: binance_client::usdm::wire::Kline =
            serde_json::from_value(columns.clone()).unwrap();
        assert_eq!(serde_json::to_value(kline).unwrap(), columns);
    }

    #[test]
    fn algo_acknowledgments_require_an_identity() {
        assert!(
            serde_json::from_value::<binance_client::usdm::rest_models::NewAlgoOrderResponse>(
                json!({"algoStatus":"NEW"})
            )
            .is_err()
        );
    }

    #[test]
    fn rpi_limit_orders_and_conditional_test_requests_remain_available() {
        assert!(
            order()
                .type_value("LIMIT")
                .time_in_force("RPI")
                .price(dec("100"))
                .build()
                .is_ok()
        );
        let test = binance_client::usdm::rest_requests::TestOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("STOP_MARKET")
            .quantity(dec("0.001"))
            .stop_price(dec("100"))
            .new_client_order_id(ClientOrderId::new("test-only-1").unwrap());
        assert!(test.build().is_ok());
        assert!(
            serde_json::from_value::<binance_client::usdm::rest_models::TestOrderResponse>(json!(
                {}
            ))
            .is_ok()
        );
    }

    #[test]
    fn zeroed_test_order_acknowledgment_decodes_as_validated_evidence() {
        // Observed on demo-fapi 2026-10-03 (issue #47): the validation-only
        // response zeroes every field the venue does not echo. Identity
        // strings stay plain evidence and empty amounts stay absent rather
        // than failing the whole acknowledgment.
        let ack: binance_client::usdm::rest_models::TestOrderResponse =
            serde_json::from_value(json!({
                "orderId":0,"symbol":"","status":"","clientOrderId":"","price":"","origQty":"",
                "executedQty":"","timeInForce":"","type":"","reduceOnly":false,
                "closePosition":false,"side":"","stopPrice":"","priceProtect":false,
                "origType":"","updateTime":0
            }))
            .unwrap();
        assert_eq!(ack.order_id, Some(0));
        assert_eq!(ack.symbol.as_deref(), Some(""));
        assert_eq!(ack.client_order_id.as_deref(), Some(""));
        assert_eq!(ack.price, None);
        assert_eq!(ack.executed_qty, None);
        assert_eq!(ack.stop_price, None);
        assert_eq!(ack.update_time, Some(0));
        // A populated echo decodes with exact amounts, and malformed
        // financial data stays refused (see also order_wire_contract).
        let real: binance_client::usdm::rest_models::TestOrderResponse =
            serde_json::from_value(json!({
                "orderId":9,"symbol":"BTCUSDT","clientOrderId":"caller-1","price":"100.5",
                "executedQty":"0.001","status":"NEW","side":"BUY","updateTime":17
            }))
            .unwrap();
        assert_eq!(real.symbol.as_deref(), Some("BTCUSDT"));
        assert_eq!(real.price, Some(dec("100.5")));
        assert_eq!(real.executed_qty, Some(dec("0.001")));
    }

    #[test]
    fn malformed_batch_members_preserve_valid_receipts_and_negative_codes_win() {
        use binance_client::usdm::{rest_models::NewOrderResponse, wire::BatchResult};
        let members: Vec<BatchResult<NewOrderResponse>> = serde_json::from_value(json!([
            {"orderId":7},
            {"orderId":8,"executedQty":"malformed"},
            {"orderId":9,"code":-2010,"msg":"Rejected"},
            {"code":-9999,"msg":"future code"}
        ]))
        .unwrap();
        assert!(matches!(&members[0],BatchResult::Success(v) if v.order_id==7));
        assert!(matches!(&members[1],BatchResult::Unknown(v) if v.as_value()["orderId"]==8));
        assert!(
            matches!(&members[2],BatchResult::Failure(v) if v.code==-2010 && v.extra.as_value()["orderId"]==9)
        );
        assert!(
            matches!(&members[3],BatchResult::Failure(v) if v.outcome()==binance_client::Outcome::Unknown)
        );
    }
}
