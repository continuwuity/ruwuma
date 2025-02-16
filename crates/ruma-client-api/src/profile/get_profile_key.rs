//! `GET /_matrix/client/*/profile/{user_id}/{key_name}`
//!
//! Gets a custom profile key from the user

pub mod unstable {
    //! `msc4133` ([MSC])
    //!
    //! [MSC]: https://github.com/tcpipuk/matrix-spec-proposals/blob/main/proposals/4133-extended-profiles.md

    use std::collections::BTreeMap;

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedUserId,
    };
    use serde_json::Value as JsonValue;

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: None,
        history: {
            unstable => "/_matrix/client/unstable/uk.tcpip.msc4133/profile/:user_id/:key_name",
            1.13 => "/_matrix/client/v3/profile/:user_id/:key_name",
        }
    };

    #[request(error = crate::Error)]
    pub struct Request {
        #[ruma_api(path)]
        pub user_id: OwnedUserId,

        #[ruma_api(path)]
        pub key_name: String,
    }

    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {
        #[ruma_api(body)]
        pub value: BTreeMap<String, JsonValue>,
    }

    impl Request {
        pub fn new(user_id: OwnedUserId, key_name: String) -> Self {
            Self { user_id, key_name }
        }
    }

    impl Response {
        pub fn new(value: BTreeMap<String, JsonValue>) -> Self {
            Self { value }
        }
    }
}
