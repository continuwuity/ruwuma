pub mod unstable {
	//! Unstable features for the authenticated media redact API.

	use ruma_common::{api::{request, response, Metadata}, metadata, OwnedServerName};

	const METADATA: Metadata = metadata! {
		method: POST,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			unstable => "/_matrix/client/unstable/uk.timedout.msc4322/media/redact/:server_name/:media_id",
		}
	};

	/// Request type for the `redact_event` endpoint.
	#[request(error = crate::Error)]
	pub struct Request {
		/// The server name hosting the media.
		#[ruma_api(path)]
		pub server_name: OwnedServerName,

		/// The media ID to redact.
		#[ruma_api(path)]
		pub media_id: String,
	}

	/// Response type for the `redact_event` endpoint.
	#[response(error = crate::Error)]
	#[derive(Default)]
	pub struct Response {}
}