//! `POST /api/1/spam_check/user_may_join_room`
//!
//! Endpoint that checks whether a user may join a given room via Draupnir anti-spam

pub mod v1 {
    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedRoomId, OwnedUserId,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: AppserviceToken,
        history: {
            1.0 => "/api/1/spam_check/user_may_join_room",
        }
    };

    /// Request type for the `user_may_join_room` callback.
    #[request]
    pub struct Request {
        /// The user trying to join a room
        pub user_id: OwnedUserId,
        /// The room the user is trying to join
        pub room_id: OwnedRoomId,
        /// Whether the user was invited to this room
        pub is_invited: bool,
    }

    /// Response type for the `user_may_join_room` callback.
    #[response]
    #[derive(Default)]
    pub struct Response {}

    impl Request {
        /// Creates a new empty `Request`.
        pub fn new(user_id: OwnedUserId, room_id: OwnedRoomId, is_invited: bool) -> Self {
            Self { user_id, room_id, is_invited }
        }
    }

    impl Response {
        /// Creates a new empty `Response`.
        pub fn new() -> Self {
            Self::default()
        }
    }
}
