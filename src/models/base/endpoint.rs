#[derive(Debug)]
pub(crate) struct Endpoint {
    pub(crate) path: String,
    pub(crate) method: reqwest::Method,
    pub(crate) auth: bool,
}
