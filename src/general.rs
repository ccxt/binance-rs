use error_chain::bail;

use crate::model::{Empty, ExchangeInformation, ServerTime, Symbol};
use crate::client::Client;
use crate::errors::Result;
use crate::api::API;
use crate::api::Spot;
use crate::util::{build_request, json_string_array};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client.get::<Empty>(API::Spot(Spot::Ping), None)?;
        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Spot(Spot::Time), None)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(API::Spot(Spot::ExchangeInfo), None)
    }

    /// Exchange information filtered by symbol(s) and/or symbol status.
    ///
    /// - `symbols`: restrict the response to these symbols (an unknown symbol is an error)
    /// - `symbol_status`: e.g. `TRADING`, `HALT`, `BREAK`, `CANCEL_ONLY`; cannot be combined
    ///   with `symbols`
    /// - `show_permission_sets`: set to `false` to omit `permissionSets` (default `true`)
    pub fn exchange_info_with_params<S, T, P>(
        &self, symbols: Option<&[S]>, symbol_status: T, show_permission_sets: P,
    ) -> Result<ExchangeInformation>
    where
        S: AsRef<str>,
        T: Into<Option<String>>,
        P: Into<Option<bool>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbols) = symbols {
            if symbols.len() == 1 {
                parameters.insert("symbol".into(), symbols[0].as_ref().to_string());
            } else {
                parameters.insert("symbols".into(), json_string_array(symbols));
            }
        }
        if let Some(status) = symbol_status.into() {
            parameters.insert("symbolStatus".into(), status);
        }
        if let Some(show) = show_permission_sets.into() {
            parameters.insert("showPermissionSets".into(), show.to_string());
        }
        let request = build_request(parameters);
        self.client
            .get(API::Spot(Spot::ExchangeInfo), Some(request))
    }

    // Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }
}
