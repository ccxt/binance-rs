use crate::model::Empty;
use crate::futures::model::{ExchangeInformation, ServerTime, Symbol};
use crate::client::Client;
use crate::errors::{Result, SdkError};
use crate::api::API;
use crate::api::Futures;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client
            .get::<Empty>(API::Futures(Futures::Ping), None)?;
        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Futures(Futures::Time), None)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(API::Futures(Futures::ExchangeInfo), None)
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
                Err(SdkError::Other("Symbol not found".into()))
            }
            Err(e) => Err(e),
        }
    }
}
