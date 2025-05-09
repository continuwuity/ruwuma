//! `PUT /_matrix/app/*/org.continuwuity.callback/user_may_join_room`
//!
//! Endpoint to trigger AS callbacks.

pub mod unstable {
    //! `/unstable/`

    use ruma_common::{api::{request, response, Metadata}, metadata, OwnedRoomId, OwnedUserId};

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/app/unstable/org.continuwuity.callback/user_may_join_room",
        }
    };

    /// Request type for the `callback` endpoint.
    #[request]
    pub struct Request {
        /// The user ID of the user who is joining the room
        pub user: OwnedUserId,

        /// The destination room ID to where the user is joining
        pub room_id: OwnedRoomId,

        /// Whether the user is invited to the room they are joining
        pub is_invited: bool,
    }

    /// Response type for the `callback` endpoint.
    #[response]
    #[derive(Default)]
    pub struct Response {
        /// Whether the invite is allowed or not.
        pub ok: bool,

        /// If ok is false, the reason as to why the invite is not allowed.
        ///
        /// This is an optional field, and may be omitted if the reason is not known.
        pub reason: Option<String>,
    }

    impl Request {
        /// Creates a new empty `Request`.
        pub fn new(user: OwnedUserId, room_id: OwnedRoomId, is_invited: bool) -> Self {
            Self { user, room_id, is_invited }
        }
    }

    impl Response {
        /// Creates a new empty `Response`.
        pub fn new() -> Self {
            Self::default()
        }
    }
}
