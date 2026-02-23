pub mod v1 {
	use ruma_common::{api::{request, response, Metadata}, metadata, OwnedRoomOrAliasId};

	const METADATA: Metadata = metadata! {
		method: POST,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			1.0 => "/_continuwuity/admin/v1/users/create",
		}
	};

	#[request]
	pub struct Request {
		pub localpart: String,
		pub password: String,

		#[serde(default)]
		pub admin: bool,
		#[serde(default)]
		pub suspended: bool,
		#[serde(default)]
		pub locked: bool,
		#[serde(default)]
		pub disable_login: bool,
		#[serde(default)]
		pub additional_auto_join_rooms: Option<Vec<OwnedRoomOrAliasId>>
	}

	#[response]
	#[derive(Default)]
	pub struct Response {}
}
