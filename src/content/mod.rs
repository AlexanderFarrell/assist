use rocket::Route;
use crate::auto::api::{ApiAction, DataAction};
use crate::auto::client::ClientDisplay;

pub struct Content<T> {
    adds: Vec<dyn Add<T>>,
    removes: Vec<dyn Remove<T>>,
    searches: Vec<dyn Search<T>>,
    updates: Vec<dyn Update<T>>,
    require_auth: bool,
}

impl<T> Content<T> {
    pub fn new(require_auth: bool) -> Self {
        Self {
            adds: vec![],
            removes: vec![],
            searches: vec![],
            updates: vec![],
            require_auth
        }
    }
}

pub trait Add<T>: ApiAction + DataAction {}
pub trait Remove<T>: ApiAction + DataAction {}
pub trait Search<T>: ApiAction + ClientDisplay + DataAction {}
pub trait Update<T>: ApiAction + DataAction {}