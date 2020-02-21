
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FredError {
    pub(crate) error_code: usize,
    pub(crate) error_message: String,
}