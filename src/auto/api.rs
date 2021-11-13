//! A helper module for building routes automatically.
//!
//!  - Assists in building queries & executions to a **database**
//!  - Assists in building HTTP routes from a **server**
//!  - Assists in building HTTP routes to a **client**
//!

use rocket::Route;

/// Implement for actions which should communicate with a database.
pub trait DataAction {
    ///Returns the sql for the element
    fn sql() -> &'static str;
}

/// Implement for actions which should communicate between server & client via HTTP.
pub trait ApiAction {
    /// Returns how the server should handle requests for this action.
    fn server_route() -> Route;

    /// Returns how the client should invoke the action
    fn client_route() -> &'static str;
}

/// Implement for actions which should communicate peer to peer.
trait ApiWebSocketAction {
    //server()
    //client()
}