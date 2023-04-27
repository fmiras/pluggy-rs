use serde::Deserialize;

pub use crate::resources::category::*;
pub use crate::resources::connector::*;
pub use crate::resources::execution::*;
pub use crate::resources::item::*;
pub use crate::resources::validation::*;

mod category;
mod connector;
mod execution;
mod item;
mod validation;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub results: Vec<T>,
    pub page: i32,
    pub total_pages: i32,
    pub total: i32,
}
