//! `GET /_continuwuity/admin/users/list`
//!
//! Lists all users on this homeserver.

pub mod v1 {
	use ruma_common::{api::{request, response, Metadata}, metadata, OwnedRoomOrAliasId, OwnedUserId};

	const METADATA: Metadata = metadata! {
		method: POST,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			1.0 => "/_continuwuity/admin/users/create",
		}
	};

	#[request]
	pub struct Request {
		/// The desired user ID localpart
		pub localpart: String,
		/// The user's password
		pub password: String,
		/// The user's display name, if any
		pub display_name: Option<String>,
		/// The user's avatar URL, if any
		pub avatar_url: Option<String>,
		/// Whether this user should be made an admin.
		pub is_admin: bool,
		/// Whether this user is deactivated.
		pub is_suspended: bool,
		/// Whether this user is locked.
		pub is_locked: bool,
		/// Whether this user is forbidden from logging in.
		pub is_login_forbidden: bool,

		/// A list of rooms to join the user to upon creation.
		/// This is in addition to the server-configured auto_join_rooms.
		pub join_to: Option<Vec<OwnedRoomOrAliasId>>
	}

	#[response]
	pub struct Response {
		/// The created user
		pub user_id: OwnedUserId,
	}
	
	impl Request {
		pub fn new(
			localpart: String,
			password: String,
			display_name: Option<String>,
			avatar_url: Option<String>,
			is_admin: bool,
			is_suspended: bool,
			is_locked: bool,
			is_login_forbidden: bool,
			join_to: Option<Vec<OwnedRoomOrAliasId>>,
		) -> Self {
			Self {
				localpart,
				password,
				display_name,
				avatar_url,
				is_admin,
				is_suspended,
				is_locked,
				is_login_forbidden,
				join_to,
			}
		}
	}
	
	impl Response {
		pub fn new(user_id: OwnedUserId) -> Self {
			Self { user_id }
		}
	}
}