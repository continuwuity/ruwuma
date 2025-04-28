//! `DELETE /_matrix/client/*/profile/{userId}/m.tz`
//!
//! Deletes the timezone key of the user.

pub mod unstable {
    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedUserId,
    };

    const METADATA: Metadata = metadata! {
        method: DELETE,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/uk.tcpip.msc4133/profile/{user_id}/us.cloke.msc4175.tz",
            1.13 => "/_matrix/client/v3/profile/{user_id}/m.tz",
        }
    };

    /// Request type for the `delete_timezone_key` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The user whose timezone will be deleted.
        #[ruma_api(path)]
        pub user_id: OwnedUserId,
    }

    /// Response type for the `delete_timezone_key` endpoint.
    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {}

    impl Request {
        /// Creates a new `Request` with the given user ID.
        pub fn new(user_id: OwnedUserId) -> Self {
            Self { user_id }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
