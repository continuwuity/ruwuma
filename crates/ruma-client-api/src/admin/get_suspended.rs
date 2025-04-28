//! `GET /_matrix/client/v1/admin/suspend/{userId}`
//!
//! Check the suspension status of a target user

pub mod v1 {
    //! `/_matrix/client/unstable/uk.timedout.msc4323/admin/suspend/{userID}` ([msc])
    //!
    //! [msc]: https://github.com/matrix-org/matrix-spec-proposals/pull/4323

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedUserId,
    };
    use serde::{Deserialize, Serialize};

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/uk.timedout.msc4323/admin/suspend/{user_id}",
        }
    };

    /// Request type for the get & set user suspension status endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The user to look up.
        #[ruma_api(path)]
        pub user_id: OwnedUserId,
    }

    /// Response type for the suspension endpoints
    #[response(error = crate::Error)]
    pub struct Response {
        /// Whether the user is currently suspended.
        pub suspended: bool,
    }

    impl Request {
        /// Creates a new `Request` with the given user id.
        pub fn new(user_id: OwnedUserId) -> Self {
            Self { user_id }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given suspension status.
        pub fn new(suspended: bool) -> Self {
            Self { suspended }
        }
    }
}