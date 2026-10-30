use crate::model::{Success, UserDataStream};
use crate::client::Client;
use crate::errors::Result;
use crate::api::API;
use crate::api::Spot;

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

/// Legacy `listenKey` based user data stream management.
///
/// Binance retired `POST`/`PUT`/`DELETE /api/v3/userDataStream` on 2026-02-20. Spot user
/// data must now be consumed through the WebSocket API; see
/// [`crate::websockets::WebSockets::connect_user_data_stream`]. These methods are kept
/// for API compatibility only and will fail against production.
impl UserStream {
    // User Stream
    #[deprecated(
        note = "retired by Binance on 2026-02-20, use WebSockets::connect_user_data_stream"
    )]
    pub fn start(&self) -> Result<UserDataStream> {
        self.client.post(API::Spot(Spot::UserDataStream))
    }

    #[deprecated(
        note = "retired by Binance on 2026-02-20, use WebSockets::connect_user_data_stream"
    )]
    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(API::Spot(Spot::UserDataStream), listen_key)
    }

    #[deprecated(
        note = "retired by Binance on 2026-02-20, use WebSockets::connect_user_data_stream"
    )]
    pub fn close(&self, listen_key: &str) -> Result<Success> {
        self.client
            .delete(API::Spot(Spot::UserDataStream), listen_key)
    }
}
