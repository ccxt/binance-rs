use crate::account::Account;
use crate::client::Client;
use crate::config::{
    Config, FUTURES_MAINNET, FUTURES_TESTNET, FUTURES_WS_MAINNET, FUTURES_WS_TESTNET, SPOT_MAINNET,
    SPOT_TESTNET, SPOT_WS_MAINNET, SPOT_WS_TESTNET,
};
use crate::futures::account::FuturesAccount;
use crate::futures::general::FuturesGeneral;
use crate::futures::market::FuturesMarket;
use crate::futures::userstream::FuturesUserStream;
use crate::general::General;
use crate::market::Market;
use crate::userstream::UserStream;
use crate::savings::Savings;

#[allow(clippy::all)]
pub enum API {
    Spot(Spot),
    Savings(Sapi),
    Futures(Futures),
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    UiKlines,
    AvgPrice,
    Ticker24hr,
    TickerTradingDay,
    /// Rolling window price change statistics (`GET /api/v3/ticker`)
    Ticker,
    Price,
    BookTicker,
    Order,
    OrderTest,
    OrderCancelReplace,
    OrderAmendKeepPriority,
    OpenOrders,
    AllOrders,
    /// Deprecated by Binance in favour of [`Spot::OrderListOco`]
    Oco,
    OrderList,
    OrderListOco,
    OrderListOto,
    OrderListOtoco,
    OrderListOpo,
    OrderListOpoco,
    AllOrderList,
    OpenOrderList,
    SorOrder,
    SorOrderTest,
    Account,
    AccountCommission,
    MyTrades,
    RateLimitOrder,
    MyPreventedMatches,
    MyAllocations,
    OrderAmendments,
    MyFilters,
    /// Retired by Binance on 2026-02-20; use the WebSocket API user data stream instead.
    UserDataStream,
}

pub enum Sapi {
    AllCoins,
    AssetDetail,
    DepositAddress,
    SpotFuturesTransfer,
}

pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    RpiDepth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndexKlines,
    PremiumIndex,
    FundingRate,
    FundingInfo,
    Ticker24hr,
    TickerPrice,
    TickerPriceV2,
    BookTicker,
    /// Removed by Binance; `/fapi/v1/allForceOrders` no longer exists. Use [`Futures::ForceOrders`].
    AllForceOrders,
    ForceOrders,
    AllOpenOrders,
    AllOrders,
    UserTrades,
    Order,
    OpenOrder,
    BatchOrders,
    CountdownCancelAll,
    OrderAmendment,
    AlgoOrder,
    AlgoOpenOrders,
    OpenAlgoOrders,
    AllAlgoOrders,
    /// Deprecated by Binance in favour of [`Futures::PositionRiskV3`]
    PositionRisk,
    PositionRiskV3,
    /// Deprecated by Binance in favour of [`Futures::BalanceV3`]
    Balance,
    BalanceV3,
    PositionSide,
    MultiAssetsMargin,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    Basis,
    LvtKlines,
    IndexInfo,
    AssetIndex,
    Constituents,
    InsuranceBalance,
    SymbolAdlRisk,
    DeliveryPrice,
    TradingSchedule,
    ChangeInitialLeverage,
    MarginType,
    PositionMargin,
    PositionMarginHistory,
    /// Deprecated by Binance in favour of [`Futures::AccountV3`]
    Account,
    AccountV3,
    AccountConfig,
    SymbolConfig,
    CommissionRate,
    LeverageBracket,
    AdlQuantile,
    ApiTradingStatus,
    FeeBurn,
    RateLimitOrder,
    OpenOrders,
    UserDataStream,
    Income,
    IncomeAsyn,
    IncomeAsynId,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(route) => match route {
                Spot::Ping => "/api/v3/ping",
                Spot::Time => "/api/v3/time",
                Spot::ExchangeInfo => "/api/v3/exchangeInfo",
                Spot::Depth => "/api/v3/depth",
                Spot::Trades => "/api/v3/trades",
                Spot::HistoricalTrades => "/api/v3/historicalTrades",
                Spot::AggTrades => "/api/v3/aggTrades",
                Spot::Klines => "/api/v3/klines",
                Spot::UiKlines => "/api/v3/uiKlines",
                Spot::AvgPrice => "/api/v3/avgPrice",
                Spot::Ticker24hr => "/api/v3/ticker/24hr",
                Spot::TickerTradingDay => "/api/v3/ticker/tradingDay",
                Spot::Ticker => "/api/v3/ticker",
                Spot::Price => "/api/v3/ticker/price",
                Spot::BookTicker => "/api/v3/ticker/bookTicker",
                Spot::Order => "/api/v3/order",
                Spot::OrderTest => "/api/v3/order/test",
                Spot::OrderCancelReplace => "/api/v3/order/cancelReplace",
                Spot::OrderAmendKeepPriority => "/api/v3/order/amend/keepPriority",
                Spot::OpenOrders => "/api/v3/openOrders",
                Spot::AllOrders => "/api/v3/allOrders",
                Spot::Oco => "/api/v3/order/oco",
                Spot::OrderList => "/api/v3/orderList",
                Spot::OrderListOco => "/api/v3/orderList/oco",
                Spot::OrderListOto => "/api/v3/orderList/oto",
                Spot::OrderListOtoco => "/api/v3/orderList/otoco",
                Spot::OrderListOpo => "/api/v3/orderList/opo",
                Spot::OrderListOpoco => "/api/v3/orderList/opoco",
                Spot::AllOrderList => "/api/v3/allOrderList",
                Spot::OpenOrderList => "/api/v3/openOrderList",
                Spot::SorOrder => "/api/v3/sor/order",
                Spot::SorOrderTest => "/api/v3/sor/order/test",
                Spot::Account => "/api/v3/account",
                Spot::AccountCommission => "/api/v3/account/commission",
                Spot::MyTrades => "/api/v3/myTrades",
                Spot::RateLimitOrder => "/api/v3/rateLimit/order",
                Spot::MyPreventedMatches => "/api/v3/myPreventedMatches",
                Spot::MyAllocations => "/api/v3/myAllocations",
                Spot::OrderAmendments => "/api/v3/order/amendments",
                Spot::MyFilters => "/api/v3/myFilters",
                Spot::UserDataStream => "/api/v3/userDataStream",
            },
            API::Savings(route) => match route {
                Sapi::AllCoins => "/sapi/v1/capital/config/getall",
                Sapi::AssetDetail => "/sapi/v1/asset/assetDetail",
                Sapi::DepositAddress => "/sapi/v1/capital/deposit/address",
                Sapi::SpotFuturesTransfer => "/sapi/v1/futures/transfer",
            },
            API::Futures(route) => match route {
                Futures::Ping => "/fapi/v1/ping",
                Futures::Time => "/fapi/v1/time",
                Futures::ExchangeInfo => "/fapi/v1/exchangeInfo",
                Futures::Depth => "/fapi/v1/depth",
                Futures::RpiDepth => "/fapi/v1/rpiDepth",
                Futures::Trades => "/fapi/v1/trades",
                Futures::HistoricalTrades => "/fapi/v1/historicalTrades",
                Futures::AggTrades => "/fapi/v1/aggTrades",
                Futures::Klines => "/fapi/v1/klines",
                Futures::ContinuousKlines => "/fapi/v1/continuousKlines",
                Futures::IndexPriceKlines => "/fapi/v1/indexPriceKlines",
                Futures::MarkPriceKlines => "/fapi/v1/markPriceKlines",
                Futures::PremiumIndexKlines => "/fapi/v1/premiumIndexKlines",
                Futures::PremiumIndex => "/fapi/v1/premiumIndex",
                Futures::FundingRate => "/fapi/v1/fundingRate",
                Futures::FundingInfo => "/fapi/v1/fundingInfo",
                Futures::Ticker24hr => "/fapi/v1/ticker/24hr",
                Futures::TickerPrice => "/fapi/v1/ticker/price",
                Futures::TickerPriceV2 => "/fapi/v2/ticker/price",
                Futures::BookTicker => "/fapi/v1/ticker/bookTicker",
                Futures::AllForceOrders => "/fapi/v1/allForceOrders",
                Futures::ForceOrders => "/fapi/v1/forceOrders",
                Futures::AllOpenOrders => "/fapi/v1/allOpenOrders",
                Futures::AllOrders => "/fapi/v1/allOrders",
                Futures::UserTrades => "/fapi/v1/userTrades",
                Futures::PositionSide => "/fapi/v1/positionSide/dual",
                Futures::MultiAssetsMargin => "/fapi/v1/multiAssetsMargin",
                Futures::Order => "/fapi/v1/order",
                Futures::OpenOrder => "/fapi/v1/openOrder",
                Futures::BatchOrders => "/fapi/v1/batchOrders",
                Futures::CountdownCancelAll => "/fapi/v1/countdownCancelAll",
                Futures::OrderAmendment => "/fapi/v1/orderAmendment",
                Futures::AlgoOrder => "/fapi/v1/algoOrder",
                Futures::AlgoOpenOrders => "/fapi/v1/algoOpenOrders",
                Futures::OpenAlgoOrders => "/fapi/v1/openAlgoOrders",
                Futures::AllAlgoOrders => "/fapi/v1/allAlgoOrders",
                Futures::PositionRisk => "/fapi/v2/positionRisk",
                Futures::PositionRiskV3 => "/fapi/v3/positionRisk",
                Futures::Balance => "/fapi/v2/balance",
                Futures::BalanceV3 => "/fapi/v3/balance",
                Futures::OpenInterest => "/fapi/v1/openInterest",
                Futures::OpenInterestHist => "/futures/data/openInterestHist",
                Futures::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio",
                Futures::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio",
                Futures::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio",
                Futures::TakerlongshortRatio => "/futures/data/takerlongshortRatio",
                Futures::Basis => "/futures/data/basis",
                Futures::LvtKlines => "/fapi/v1/lvtKlines",
                Futures::IndexInfo => "/fapi/v1/indexInfo",
                Futures::AssetIndex => "/fapi/v1/assetIndex",
                Futures::Constituents => "/fapi/v1/constituents",
                Futures::InsuranceBalance => "/fapi/v1/insuranceBalance",
                Futures::SymbolAdlRisk => "/fapi/v1/symbolAdlRisk",
                Futures::DeliveryPrice => "/futures/data/delivery-price",
                Futures::TradingSchedule => "/fapi/v1/tradingSchedule",
                Futures::ChangeInitialLeverage => "/fapi/v1/leverage",
                Futures::MarginType => "/fapi/v1/marginType",
                Futures::PositionMargin => "/fapi/v1/positionMargin",
                Futures::PositionMarginHistory => "/fapi/v1/positionMargin/history",
                Futures::Account => "/fapi/v2/account",
                Futures::AccountV3 => "/fapi/v3/account",
                Futures::AccountConfig => "/fapi/v1/accountConfig",
                Futures::SymbolConfig => "/fapi/v1/symbolConfig",
                Futures::CommissionRate => "/fapi/v1/commissionRate",
                Futures::LeverageBracket => "/fapi/v1/leverageBracket",
                Futures::AdlQuantile => "/fapi/v1/adlQuantile",
                Futures::ApiTradingStatus => "/fapi/v1/apiTradingStatus",
                Futures::FeeBurn => "/fapi/v1/feeBurn",
                Futures::RateLimitOrder => "/fapi/v1/rateLimit/order",
                Futures::OpenOrders => "/fapi/v1/openOrders",
                Futures::UserDataStream => "/fapi/v1/listenKey",
                Futures::Income => "/fapi/v1/income",
                Futures::IncomeAsyn => "/fapi/v1/income/asyn",
                Futures::IncomeAsynId => "/fapi/v1/income/asyn/id",
            },
        })
    }
}

pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self;
    fn set_verbose(&mut self, verbose: bool);
    fn set_testnet(&mut self, testnet: bool);
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}

impl Binance for Savings {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_TESTNET.into());
        } else {
            self.client.set_host(SPOT_MAINNET.into());
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(SPOT_WS_TESTNET.into());
        } else {
            self.client.set_host(SPOT_WS_MAINNET.into());
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesGeneral {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(FUTURES_TESTNET.into());
        } else {
            self.client.set_host(FUTURES_MAINNET.into());
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesMarket {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(FUTURES_TESTNET.into());
        } else {
            self.client.set_host(FUTURES_MAINNET.into());
        }
    }
}

impl Binance for FuturesAccount {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self {
        Self {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(FUTURES_TESTNET.into());
        } else {
            self.client.set_host(FUTURES_MAINNET.into());
        }
    }
}

impl Binance for FuturesUserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesUserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesUserStream {
        FuturesUserStream {
            client: Client::new(
                api_key,
                secret_key,
                config.futures_rest_api_endpoint.clone(),
            ),
            recv_window: config.recv_window,
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }

    fn set_testnet(&mut self, testnet: bool) {
        if testnet {
            self.client.set_host(FUTURES_WS_TESTNET.into());
        } else {
            self.client.set_host(FUTURES_WS_MAINNET.into());
        }
    }
}
