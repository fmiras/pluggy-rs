use serde::Deserialize;

pub use crate::resources::category::*;
pub use crate::resources::connector::*;
pub use crate::resources::execution::*;
pub use crate::resources::item::*;
pub use crate::resources::validation::*;
pub use crate::resources::webhook::*;

mod category;
mod connector;
mod execution;
mod item;
mod validation;
mod webhook;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub results: Vec<T>,
    pub page: i32,
    pub total_pages: i32,
    pub total: i32,
}
