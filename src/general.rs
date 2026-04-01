use error_chain::bail;
use crate::util::build_request;
use crate::model::{Empty, ExchangeInformation, ServerTime, Symbol};
use crate::client::Client;
use crate::errors::Result;
use crate::api::API;
use crate::api::Spot;
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



    // Obtain exchange information with permissions
    // - Current exchange trading rules and symbol information
    pub fn exchange_info_permission<S>(&self, permission : S) -> Result<ExchangeInformation>
    where
        S: Into<String>, 
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("permissions".into(), permission.into());
        let request = build_request(parameters);
        self.client.get(API::Spot(Spot::ExchangeInfo), Some(request))
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
