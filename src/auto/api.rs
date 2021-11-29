//! A helper module for building routes automatically.
//!
//!  - Assists in building queries & executions to a **database**
//!  - Assists in building HTTP routes from a **server**
//!  - Assists in building HTTP routes to a **client**
//!

use crate::p
use std::borrow::Borrow;
use rocket::{Build, Rocket, Route};
use rocket_sync_db_pools::postgres::Row;

pub trait Rowable {
    fn from_row(row: &Row) -> Self;
}

/// Implement for actions which should communicate with a database.
pub trait DataQuery<T> {
    ///Returns the sql for the element
    fn sql(&self) -> Result<Vec<T>, &'static str>;
}

/// Implement for actions which should send information to the database.
pub trait DataExecute {
    ///Returns the sql for the element
    fn sql(&self) -> Result<(), &'static str>;
}

/// Implement for actions which should communicate between server & client via HTTP.
pub trait ApiAction {
    fn server_route_path(&self) -> &'static str;

    /// Returns how the server should handle requests for this action.
    fn server_route(&self) -> Route;

    /// Returns how the client should invoke the action
    fn client_route(&self) -> &'static str;
}

/// Implement for actions which should communicate peer to peer.
trait ApiWebSocketAction {
    //server()
    //client()
}

/// Build routes on the Rocket application
fn build_routes(mut rocket: Rocket<Build>, routes: &[&impl ApiAction]) -> Rocket<Build> {
    for route in routes.iter() {
        rocket = rocket.mount(route.server_route_path(), vec![route.server_route()]);
    }

    rocket
}