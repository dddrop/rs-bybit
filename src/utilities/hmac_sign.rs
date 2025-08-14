use hmac::{Hmac, Mac};
use sha2::Sha256;

pub(crate) fn get_signature(api_secret: &str, params: Vec<&str>) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).unwrap();
    for param in params {
        mac.update(param.as_bytes());
    }
    hex::encode(mac.finalize().into_bytes())
}
