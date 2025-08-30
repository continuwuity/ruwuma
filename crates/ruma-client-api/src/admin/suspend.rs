//! `GET /_matrix/client/v1/admin/suspend/{userId}`
//!
//! Check and set the suspension status of a target user

pub mod v3 {
    //! `/_matrix/client/unstable/uk.timedout.msc4323/` ([msc])
    //!
    //! [msc]: https://github.com/matrix-org/matrix-spec-proposals/pull/4323

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedUserId,
    };
    use serde::{Deserialize, Serialize};

    const METADATA: Metadata = metadata! {
        method: GET, // OR PUT
        rate_limited: false,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/uk.timedout.msc4323/suspend/:user_id",
        }
    };

    /// Request type for the get & set user suspension status endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The user to look up.
        #[ruma_api(path)]
        pub user_id: OwnedUserId,

        /// Whether to suspend (true) or unsuspend (false) the user. None if just checking status.
        pub suspended: Option<bool>,
    }

    /// Response type for the suspension endpoints
    #[response(error = crate::Error)]
    pub struct Response {
        /// Whether the user is currently suspended.
        pub suspended: bool,
    }

    impl Request {
        /// Creates a new `Request` with the given user id.
        pub fn new(user_id: OwnedUserId, suspended: Option<bool>) -> Self {
            Self { user_id, suspended }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given suspension status.
        pub fn new(suspended: bool) -> Self {
            Self { suspended }
        }
    }
}