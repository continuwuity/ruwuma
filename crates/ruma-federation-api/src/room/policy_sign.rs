//! `POST /_matrix/policy/unstable/org.matrix.msc4284/sign`
//!
//! Asks a policy server to sign our event

pub mod unstable {
    //! `/policy/unstable/org.matrix.msc4284` ([spec])
    //!
    //! [spec]: https://github.com/matrix-org/matrix-spec-proposals/pull/4284
    use ruma_common::{
        ServerSignatures, api::{Metadata, request, response}, metadata
    };
    use serde_json::value::RawValue as RawJsonValue;

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: ServerSignatures,
        history: {
            unstable => "/_matrix/policy/unstable/org.matrix.msc4284/sign",
        }
    };

    /// Response type for the `sign` endpoint.
    #[response]
    pub struct Response {
        /// The signatures returned from the policy server
        #[ruma_api(body)]
        pub signatures: ServerSignatures
    }

    impl Response {
        /// Creates a new `Response` with the given recommendation.
        pub fn new(signatures: ServerSignatures) -> Self {
            Self { signatures }
        }
    }

    /// Request type for the `sign` endpoint.
    #[request]
    pub struct Request {
        /// The PDU body (in canonical JSON)
        #[ruma_api(body)]
        pub pdu: Box<RawJsonValue>,
    }

    impl Request {
        /// Creates a new `Request` with the given event JSON
        pub fn new(pdu: Box<RawJsonValue>) -> Self {
            Self { pdu }
        }
    }
}