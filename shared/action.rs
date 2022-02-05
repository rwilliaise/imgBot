use async_trait::async_trait;
use std::borrow::Cow;

#[async_trait]
pub trait Action {
    /// Get image data from this action as a blob.
    async fn get_data_blob<'a>(&self) -> Cow<'a, [u8]>;
}
