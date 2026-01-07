//! `GET /_continuwuity/admin/users/list`
//!
//! Lists all users on this homeserver.

pub mod v1 {
	use ruma_common::{
		api::{request, response, Metadata},
		metadata, OwnedUserId
	};

	const METADATA: Metadata = metadata! {
		method: GET,
		rate_limited: false,
		authentication: AccessToken,
		history: {
			1.0 => "/_continuwuity/admin/users/list",
		}
	};

	#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
	pub struct User {
		/// The user's full ID.
		pub user_id: OwnedUserId,
		/// The user's avatar URL, if set.
		pub avatar_url: Option<String>,
		/// The user's display name, if set.
		pub display_name: Option<String>,
		/// Indicates whether the user is deactivated.
		pub is_deactivated: bool,
		/// Indicates whether the user is suspended.
		pub is_suspended: bool,
		/// Indicates whether the user is locked.
		pub is_locked: bool,
		/// Indicates whether the user is forbidden from logging in.
		pub is_login_forbidden: bool,
		/// Indicates whether the user is an admin.
		pub is_admin: bool,
	}

	#[request]
	#[derive(Default)]
	pub struct Request {}

	#[response]
	pub struct Response {
		pub users: Vec<User>,
	}

	impl Request {
		pub fn new() -> Self {
			Self::default()
		}
	}

	impl Response {
		pub fn new(users: Vec<User>) -> Self {
			Self { users }
		}
	}
}