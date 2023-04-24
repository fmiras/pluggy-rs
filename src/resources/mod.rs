use serde::Deserialize;

pub use crate::resources::connector::*;

mod connector;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub results: Vec<T>,
    pub page: i32,
    pub total_pages: i32,
    pub total: i32,
}
