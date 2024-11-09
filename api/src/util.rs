use serde::Serialize;

pub trait AnyhowRspc<T> {
    fn anyhow_rspc(self) -> Result<T, rspc::Error>;
}

impl<T> AnyhowRspc<T> for anyhow::Result<T>
where
    T: Serialize,
{
    fn anyhow_rspc(self) -> Result<T, rspc::Error> {
        self.map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string()))
    }
}
