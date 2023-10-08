//! The network server protocol implementation, hosting a game for zero or more clients.

use std::sync::Arc;

use capnp::message::HeapAllocator;
use capnp_rpc::{pry, ImbuedMessageBuilder};
use ocg_schemas::dependencies::capnp::capability::Promise;
use ocg_schemas::dependencies::capnp::Error;
use ocg_schemas::dependencies::kstring::KString;
use ocg_schemas::schemas::network_capnp as rpc;
use ocg_schemas::schemas::network_capnp::authenticated_server_connection::{
    BootstrapGameDataParams, BootstrapGameDataResults, SendChatMessageParams, SendChatMessageResults,
};

use crate::network::PeerAddress;
use crate::GameServer;

/// An unauthenticated RPC client->server connection handler on the server side.
pub struct Client2ServerEndpoint {
    server: Arc<GameServer>,
    peer: PeerAddress,
}

/// An authenticated RPC client->server connection handler on the server side.
pub struct AuthenticatedClient2ServerEndpoint {
    server: Arc<GameServer>,
    peer: PeerAddress,
    username: KString,
    connection: ImbuedMessageBuilder<HeapAllocator>,
}

impl Client2ServerEndpoint {
    /// Constructor.
    pub fn new(server: Arc<GameServer>, peer: PeerAddress) -> Self {
        Self { server, peer }
    }

    /// The server this endpoint is associated with.
    pub fn server(&self) -> &Arc<GameServer> {
        &self.server
    }

    /// The peer address this endpoint is connected to.
    pub fn peer(&self) -> PeerAddress {
        self.peer
    }
}

impl rpc::game_server::Server for Client2ServerEndpoint {
    fn get_server_metadata(
        &mut self,
        _params: rpc::game_server::GetServerMetadataParams,
        mut results: rpc::game_server::GetServerMetadataResults,
    ) -> Promise<(), Error> {
        let title = "OCG Server";
        let subtitle = "Subtitles to be implemented!";
        let mut meta = results.get().init_metadata();
        let mut ver = meta.reborrow().init_server_version();
        ver.set_major(0);
        ver.set_minor(0);
        ver.set_patch(1);
        ver.reborrow().init_build(4).push_str("todo");
        ver.reborrow().init_prerelease(0);

        meta.reborrow().init_title(title.len() as u32).push_str(title);
        meta.reborrow().init_subtitle(subtitle.len() as u32).push_str(subtitle);
        meta.set_player_count(0);
        meta.set_player_limit(12);
        Promise::ok(())
    }

    fn ping(
        &mut self,
        params: rpc::game_server::PingParams,
        mut results: rpc::game_server::PingResults,
    ) -> Promise<(), Error> {
        let input = pry!(params.get()).get_input();
        results.get().set_output(input);
        Promise::ok(())
    }

    fn authenticate(
        &mut self,
        params: rpc::game_server::AuthenticateParams,
        mut results: rpc::game_server::AuthenticateResults,
    ) -> Promise<(), Error> {
        let params = pry!(params.get());
        let username = KString::from_ref(pry!(pry!(params.get_username()).to_str()));
        let connection: rpc::authenticated_client_connection::Client = pry!(params.get_connection());

        // TODO: validate username

        let mut client = AuthenticatedClient2ServerEndpoint {
            server: self.server.clone(),
            peer: self.peer,
            username,
            connection: ImbuedMessageBuilder::new(HeapAllocator::new()),
        };
        pry!(client.connection.set_root(connection));

        let mut result = results.get().init_conn();
        pry!(result.set_ok(capnp_rpc::new_client(client)));

        Promise::ok(())
    }
}

impl AuthenticatedClient2ServerEndpoint {
    /// The RPC instance for sending messages to the connected client.
    pub fn connection(&mut self) -> rpc::authenticated_client_connection::Client {
        self.connection
            .get_root::<rpc::authenticated_client_connection::Client>()
            .expect("Invalid message type stored")
    }
}

impl rpc::authenticated_server_connection::Server for AuthenticatedClient2ServerEndpoint {
    fn bootstrap_game_data(&mut self, _: BootstrapGameDataParams, _: BootstrapGameDataResults) -> Promise<(), Error> {
        todo!()
    }

    fn send_chat_message(&mut self, _: SendChatMessageParams, _: SendChatMessageResults) -> Promise<(), Error> {
        todo!()
    }
}
