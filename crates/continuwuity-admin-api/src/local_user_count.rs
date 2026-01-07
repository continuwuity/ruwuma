//! `GET /_continuwuity/admin/local_user_count`
//!
//! Endpoint to get the count of local users on this homeserver.

pub mod v1 {
	use ruma_common::{
		api::{request, response, Metadata},
		metadata
	};

	const METADATA: Metadata = metadata! {
		method: GET,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			1.0 => "/_continuwuity/admin/local_user_count",
		}
	};

	#[request]
	#[derive(Default)]
	pub struct Request {}

	#[response]
	pub struct Response {
		pub local_user_count: u64,
	}

	impl Request {
		pub fn new() -> Self {
			Self::default()
		}
	}

	impl Response {
		pub fn new(local_user_count: u64) -> Self {
			Self { local_user_count }
		}
	}
}