pub mod v1 {
	use ruma_common::{
		api::{request, response, Metadata},
		metadata, OwnedRoomId,
	};

	const METADATA: Metadata = metadata! {
		method: GET,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			1.0 => "/_continuwuity/admin/rooms/list",
		}
	};

	#[request]
	#[derive(Default)]
	pub struct Request {}

	#[response]
	pub struct Response {
		pub rooms: Vec<OwnedRoomId>,
	}

	impl Request {
		pub fn new() -> Self {
			Self::default()
		}
	}

	impl Response {
		pub fn new(rooms: Vec<OwnedRoomId>) -> Self {
			Self { rooms }
		}
	}
}