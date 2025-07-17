//! `POST /_matrix/client/users/{userId}/report`
//!
//! Reports an abusive user.

pub mod v3 {
    //! `MSC4260` - https://github.com/matrix-org/matrix-spec-proposals/pull/4260
    //! https://spec.matrix.org/v1.15/client-server-api/#post_matrixclientv3usersuseridreport

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedUserId,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/org.matrix.msc4260/users/:user_id/report",
            1.14 => "/_matrix/client/v3/users/:user_id/report",
        }
    };

    /// Request type for the `report_user` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The user ID being reported.
        #[ruma_api(path)]
        pub user_id: OwnedUserId,

        /// Reason to report user.
        ///
        /// May be blank.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
    }

    /// Response type for the `report_user` endpoint.
    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {}

    impl Request {
        /// Creates a new `Request` with the given user ID and reason.
        pub fn new(user_id: OwnedUserId, reason: Option<String>) -> Self {
            Self { user_id, reason }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
