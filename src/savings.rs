use crate::util::build_signed_request;
use crate::model::{
    AssetDetail, CoinInfo, DepositAddress, SpotFuturesTransferType, TradeFee, TransactionId,
    WithdrawResponse,
};
use crate::client::Client;
use crate::errors::Result;
use std::collections::BTreeMap;
use crate::api::API;
use crate::api::Sapi;

#[derive(Clone)]
pub struct Savings {
    pub client: Client,
    pub recv_window: u64,
}

impl Savings {
    /// Get all coins available for deposit and withdrawal
    pub fn get_all_coins(&self) -> Result<Vec<CoinInfo>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::AllCoins), Some(request))
    }

    /// Fetch details of assets supported on Binance.
    pub fn asset_detail(&self, asset: Option<String>) -> Result<BTreeMap<String, AssetDetail>> {
        let mut parameters = BTreeMap::new();
        if let Some(asset) = asset {
            parameters.insert("asset".into(), asset);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::AssetDetail), Some(request))
    }

    /// Maker and Taker trade fees for an asset pair
    pub fn get_trade_fee(&self, symbol: Option<String>) -> Result<Vec<TradeFee>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbol) = symbol {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::TradeFee), Some(request))
    }

    /// Fetch deposit address with network.
    ///
    /// You can get the available networks using `get_all_coins`.
    /// If no network is specified, the address for the default network is returned.
    pub fn deposit_address<S>(&self, coin: S, network: Option<String>) -> Result<DepositAddress>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("coin".into(), coin.into());
        if let Some(network) = network {
            parameters.insert("network".into(), network);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::DepositAddress), Some(request))
    }

    pub fn transfer_funds<S>(
        &self, asset: S, amount: f64, transfer_type: SpotFuturesTransferType,
    ) -> Result<TransactionId>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("asset".into(), asset.into());
        parameters.insert("amount".into(), amount.to_string());
        parameters.insert("type".into(), (transfer_type as u8).to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Savings(Sapi::SpotFuturesTransfer), request)
    }

    // Withdraw currency
    pub fn withdraw_currency<S>(
        &self, asset: S, address: S, address_tag: Option<u64>, amount: f64,
    ) -> Result<WithdrawResponse>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("asset".into(), asset.into());
        parameters.insert("address".into(), address.into());
        if let Some(address_tag) = address_tag {
            parameters.insert("addressTag".into(), address_tag.to_string());
        }
        parameters.insert("amount".into(), amount.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::Withdraw), Some(request))
    }
}
