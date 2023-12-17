use crate::{Error, Warp10Response};
use isahc::http::{HeaderMap, HeaderValue, StatusCode};

pub(crate) fn extract_header_err(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    // Extract the error header from the Warp 10 response, the header is defined here
    // https://github.com/senx/warp10-platform/blob/master/warp10/src/main/java/io/warp10/continuum/store/Constants.java#L155
    headers
        .get("X-Warp10-Error-Message")
        .map(|hv| hv.as_bytes().to_vec())
        .and_then(|buf| String::from_utf8(buf).ok())
}

pub(crate) fn handle_response(
    err: Option<String>,
    status: StatusCode,
    payload: String,
) -> crate::Result<Warp10Response> {
    let response = Warp10Response::new(status, payload);
    match response.status() {
        StatusCode::OK => Ok(response),
        _ => Err(Error::api_error(response, err)),
    }
}
