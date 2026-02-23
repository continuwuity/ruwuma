pub mod v1 {
	use std::collections::BTreeMap;
	use serde_json::Value as JsonValue;
	use ruma_common::{api::{request, response, Metadata}, metadata, OwnedRoomId};
	use ruma_events::room::member::MembershipState;

	const METADATA: Metadata = metadata! {
		method: GET,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			1.0 => "/_continuwuity/admin/v1/users/{localpart}",
		}
	};

	#[request]
	pub struct Request {
		#[ruma_api(path)]
		pub localpart: String
	}

	#[response]
	pub struct Response {
		pub admin: bool,
		pub suspended: bool,
		pub locked: bool,
		pub login_disabled: bool,
		pub profile_fields: BTreeMap<String, JsonValue>,
		pub account_data: BTreeMap<String, JsonValue>,
		pub memberships: BTreeMap<OwnedRoomId, MembershipState>,
	}
}