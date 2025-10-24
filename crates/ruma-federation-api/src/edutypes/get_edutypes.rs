//! Server EDU type filtering
pub mod unstable {
    //! spec: https://github.com/matrix-org/matrix-spec-proposals/pull/4373

    use ruma_common::{
        api::{request, response, Metadata},
        metadata
    };

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: None,
        history: {
            unstable => "/_matrix/federation/unstable/io.fsky.vel/edutypes",
            // 1.0 => "/_matrix/federation/v1/edutypes",
        }
    };

    /// Request type for the `edutypes` endpoint.
    #[request]
    pub struct Request {}

    /// Response type for the `edutypes` endpoint.
    #[response]
    pub struct Response {
        /// Whether presence EDUs should be sent/received
        #[serde(rename="m.presence", default="ruma_common::serde::default_true")]
        pub presence: bool,
        /// Whether read receipt EDUs should be sent/received
        #[serde(rename="m.receipt", default="ruma_common::serde::default_true")]
        pub receipt: bool,
        /// Whether typing EDUs should be sent/received
        #[serde(rename="m.typing", default="ruma_common::serde::default_true")]
        pub typing: bool
    }

    impl Request {
        /// Creates a new `Request` with the given event id.
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Response {
        /// Creates a new `Response` with the given server name, timestamp, and event.
        pub fn new(
            presence: bool,
            receipt: bool,
            typing: bool
        ) -> Self {
            Self { presence, receipt, typing }
        }
    }
}