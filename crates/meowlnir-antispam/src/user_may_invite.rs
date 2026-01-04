//! `POST /_meowlnir/antispam/*/user_may_invite`
//!
//! Endpoint to ping the application service.

pub mod v1 {
	use ruma_common::{api::{request, response, Metadata}, metadata, OwnedRoomId, OwnedUserId};

	const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            1.0 => "/_meowlnir/antispam/:management_room_id/user_may_invite",
        }
    };

	/// Request type for the `user_may_invite` callback.
	#[request]
	pub struct Request {
		/// The relevant management room
		#[ruma_api(path)]
		pub management_room_id: OwnedRoomId,
		/// The user sending the invite
		pub inviter: OwnedUserId,
		/// The user being invited
		pub invitee: OwnedUserId,

	}

	/// Response type for the `user_may_invite` callback.
	#[response]
	#[derive(Default)]
	pub struct Response {}

	impl Request {
		/// Creates a new empty `Request`.
		pub fn new(management_room_id: OwnedRoomId, inviter: OwnedUserId, invitee: OwnedUserId) -> Self {
			Self {
				management_room_id,
				inviter,
				invitee,
			}
		}
	}

	impl Response {
		/// Creates a new empty `Response`.
		pub fn new() -> Self {
			Self::default()
		}
	}
}
