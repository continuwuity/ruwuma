//! `GET /_matrix/client/v1/rtc/transports` ([MSC])
//!
//! Get the available MatrixRTC foci (transports) for this homeserver.
//!
//! [MSC]: https://github.com/matrix-org/matrix-spec-proposals/pull/4143

use ruma_common::{
	api::{request, response, Metadata},
	metadata,
};

use super::discover_homeserver::RtcFocusInfo;

const METADATA: Metadata = metadata! {
	method: GET,
	rate_limited: false,
	authentication: None,
	history: {
		unstable => "/_matrix/client/unstable/org.matrix.msc4143/rtc/transports",
	}
};

/// Request type for the `get_rtc_transports` endpoint.
#[request(error = crate::Error)]
#[derive(Default)]
pub struct Request {}

/// Response type for the `get_rtc_transports` endpoint.
#[response(error = crate::Error)]
pub struct Response {
	/// A list of the available MatrixRTC foci, ordered by priority.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub rtc_transports: Vec<RtcFocusInfo>,
}

impl Request {
	/// Creates an empty `Request`.
	pub fn new() -> Self {
		Self {}
	}
}

impl Response {
	/// Creates a new `Response` with the given RTC foci.
	pub fn new(rtc_transports: Vec<RtcFocusInfo>) -> Self {
		Self { rtc_transports }
	}
}
