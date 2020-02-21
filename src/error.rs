
use serde::Deserialize;

pub(crate) const TAG_NAME_REQUIRED_ERROR_TEXT: &str = "At least one tag must be specified using the tag_name() function of the related_tags::Builder.";

#[derive(Deserialize)]
pub(crate) struct FredError {
    pub(crate) error_code: usize,
    pub(crate) error_message: String,
}