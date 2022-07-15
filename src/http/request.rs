use super::method::Method;

pub struct Request {
    method: Method,
    path: String,
    query: Option<String>,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        jkjk
    }
}
