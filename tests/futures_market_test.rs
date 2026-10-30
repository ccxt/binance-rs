use binance::api::*;
use binance::config::*;
use binance::futures::market::FuturesMarket;
use binance::futures::general::FuturesGeneral;
use binance::futures::account::ContractType;
use binance::futures::model::{Filters, KlineSummaries, OpenInterestHist, Trades};

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};

    #[test]
    fn open_interest_statistics() {
        let mut server = Server::new();
        let mock_open_interest_statistics = server
            .mock("GET", "/futures/data/openInterestHist")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("limit=10&period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/open_interest_statistics.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let open_interest_hists = market
            .open_interest_statistics("BTCUSDT", "5m", 10, None, None)
            .unwrap();
        mock_open_interest_statistics.assert();

        let expectation = vec![
            OpenInterestHist {
                symbol: "BTCUSDT".into(),
                sum_open_interest: "20403.63700000".into(),
                sum_open_interest_value: "150570784.07809979".into(),
                cmc_circulating_supply: None,
                timestamp: 1583127900000,
            },
            OpenInterestHist {
                symbol: "BTCUSDT".into(),
                sum_open_interest: "20401.36700000".into(),
                sum_open_interest_value: "149940752.14464448".into(),
                cmc_circulating_supply: None,
                timestamp: 1583128200000,
            },
        ];

        assert_eq!(open_interest_hists, expectation);
    }

    #[test]
    fn funding_rate_and_mark_price() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/fapi/v1/fundingRate")
            .match_query(Matcher::Regex("limit=2&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/funding_rate.json")
            .create();
        let m2 = server
            .mock("GET", "/fapi/v1/fundingInfo")
            .with_body_from_file("tests/mocks/futures/market/funding_info.json")
            .create();
        let m3 = server
            .mock("GET", "/fapi/v1/premiumIndex")
            .match_query(Matcher::Regex("symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/mark_price.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let rates = market
            .get_funding_rate_history(Some("BTCUSDT".to_string()), None, None, 2)
            .unwrap();
        assert_eq!(rates.len(), 2);
        assert_eq!(rates[0].rate_type.as_deref(), Some("Regular"));
        assert_eq!(rates[0].funding_rate, 0.0001);

        let info = market.get_funding_info().unwrap();
        assert_eq!(info[0].funding_interval_hours, 8);

        let mark = market.get_mark_price("BTCUSDT").unwrap();
        assert_eq!(mark.index_price, Some(11781.80495970));
        assert_eq!(mark.interest_rate, Some(0.0001));

        m1.assert();
        m2.assert();
        m3.assert();
    }

    #[test]
    fn futures_data_endpoints() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/futures/data/topLongShortAccountRatio")
            .match_query(Matcher::Regex("limit=2&period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/long_short_ratio.json")
            .create();
        let m2 = server
            .mock("GET", "/futures/data/topLongShortPositionRatio")
            .match_query(Matcher::Regex("period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/long_short_ratio.json")
            .create();
        let m3 = server
            .mock("GET", "/futures/data/globalLongShortAccountRatio")
            .match_query(Matcher::Regex("period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/long_short_ratio.json")
            .create();
        let m4 = server
            .mock("GET", "/futures/data/takerlongshortRatio")
            .match_query(Matcher::Regex("period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/taker_long_short_ratio.json")
            .create();
        let m5 = server
            .mock("GET", "/futures/data/basis")
            .match_query(Matcher::Regex(
                "contractType=PERPETUAL&pair=BTCUSDT&period=5m".into(),
            ))
            .with_body_from_file("tests/mocks/futures/market/basis.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let top_accounts = market
            .top_long_short_account_ratio("BTCUSDT", "5m", 2, None, None)
            .unwrap();
        assert_eq!(top_accounts.len(), 2);
        assert_eq!(top_accounts[0].long_short_ratio, 1.8105);
        let top_positions = market
            .top_long_short_position_ratio("BTCUSDT", "5m", None, None, None)
            .unwrap();
        assert_eq!(top_positions[1].timestamp, 1583139900000);
        let global = market
            .global_long_short_account_ratio("BTCUSDT", "5m", None, None, None)
            .unwrap();
        assert_eq!(global[0].short_account, 0.3558);
        let taker = market
            .taker_long_short_ratio("BTCUSDT", "5m", None, None, None)
            .unwrap();
        assert_eq!(taker[0].buy_sell_ratio, 1.5586);
        let basis = market
            .basis("BTCUSDT", ContractType::Perpetual, "5m", None, None, None)
            .unwrap();
        assert_eq!(basis[0].contract_type, "PERPETUAL");
        assert_eq!(basis[0].basis, 13.94054945);

        for m in [m1, m2, m3, m4, m5] {
            m.assert();
        }
    }

    #[test]
    fn kline_variants() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/fapi/v1/continuousKlines")
            .match_query(Matcher::Regex(
                "contractType=PERPETUAL&interval=1m&limit=2&pair=BTCUSDT".into(),
            ))
            .with_body_from_file("tests/mocks/futures/market/continuous_klines.json")
            .create();
        let m2 = server
            .mock("GET", "/fapi/v1/indexPriceKlines")
            .match_query(Matcher::Regex("interval=1m&pair=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/continuous_klines.json")
            .create();
        let m3 = server
            .mock("GET", "/fapi/v1/markPriceKlines")
            .match_query(Matcher::Regex("interval=1m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/continuous_klines.json")
            .create();
        let m4 = server
            .mock("GET", "/fapi/v1/premiumIndexKlines")
            .match_query(Matcher::Regex("interval=1m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/continuous_klines.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let KlineSummaries::AllKlineSummaries(klines) = market
            .get_continuous_klines("BTCUSDT", ContractType::Perpetual, "1m", 2, None, None)
            .unwrap();
        assert_eq!(klines.len(), 2);
        assert_eq!(klines[0].open, "18879.99");
        market
            .get_index_price_klines("BTCUSDT", "1m", None, None, None)
            .unwrap();
        market
            .get_mark_price_klines("BTCUSDT", "1m", None, None, None)
            .unwrap();
        market
            .get_premium_index_klines("BTCUSDT", "1m", None, None, None)
            .unwrap();

        for m in [m1, m2, m3, m4] {
            m.assert();
        }
    }

    #[test]
    fn historical_trades_sends_api_key_without_signature() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/fapi/v1/historicalTrades")
            .match_header("x-mbx-apikey", "my-key")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "BTCUSDT".into()),
                Matcher::UrlEncoded("fromId".into(), "28457".into()),
                Matcher::UrlEncoded("limit".into(), "1".into()),
            ]))
            .with_body_from_file("tests/mocks/futures/market/historical_trades.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(Some("my-key".into()), None, &config);
        let Trades::AllTrades(trades) = market.get_historical_trades("BTCUSDT", 28457, 1).unwrap();
        assert_eq!(trades[0].id, 28457);
        assert_eq!(trades[0].is_rpi_trade, Some(false));
        mock.assert();
    }

    #[test]
    fn misc_market_endpoints() {
        let mut server = Server::new();
        let m1 = server
            .mock("GET", "/fapi/v1/insuranceBalance")
            .with_body_from_file("tests/mocks/futures/market/insurance_balance.json")
            .create();
        let m2 = server
            .mock("GET", "/fapi/v1/constituents")
            .match_query(Matcher::Regex("symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/index_constituents.json")
            .create();
        let m3 = server
            .mock("GET", "/fapi/v1/symbolAdlRisk")
            .match_query(Matcher::Regex("symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/symbol_adl_risk.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let insurance = market.get_insurance_balance(None).unwrap();
        assert_eq!(insurance[0].symbols[0], "BNBUSDT");
        assert_eq!(insurance[0].assets[1].asset, "USDT");

        let constituents = market.get_index_constituents("BTCUSDT").unwrap();
        assert_eq!(constituents.constituents[1].exchange, "coinbase");
        assert_eq!(constituents.constituents[1].weight, Some(0.15384615));

        let risk = market
            .get_symbol_adl_risk(Some("BTCUSDT".to_string()))
            .unwrap();
        assert_eq!(risk[0].adl_risk, "low");

        for m in [m1, m2, m3] {
            m.assert();
        }
    }

    #[test]
    fn exchange_info_tolerates_new_fields_and_filters() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/fapi/v1/exchangeInfo")
            .with_body_from_file("tests/mocks/futures/market/exchange_info.json")
            .create();
        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let general: FuturesGeneral = Binance::new_with_config(None, None, &config);
        let info = general.exchange_info().unwrap();
        mock.assert();

        assert_eq!(info.assets.len(), 3);
        assert_eq!(info.assets[2].auto_asset_exchange, None);
        let symbol = &info.symbols[0];
        assert_eq!(symbol.pair.as_deref(), Some("BLZUSDT"));
        assert_eq!(symbol.contract_type.as_deref(), Some("PERPETUAL"));
        assert_eq!(symbol.margin_asset.as_deref(), Some("USDT"));
        assert_eq!(symbol.trigger_protect, Some(0.15));
        assert_eq!(symbol.liquidation_fee, Some(0.01));
        assert_eq!(symbol.underlying_sub_type, vec!["STORAGE"]);
        assert_eq!(symbol.filters.len(), 8);
        assert!(matches!(symbol.filters[7], Filters::Unknown));
        assert!(matches!(
            symbol.filters[3],
            Filters::MaxNumOrders {
                max_num_orders: None
            }
        ));
        assert!(symbol.time_in_force.contains(&"RPI".to_string()));
    }
}
