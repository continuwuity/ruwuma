//! `PUT /_matrix/app/*/org.continuwuity.callback/user_may_invite`
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
            unstable => "/_matrix/app/unstable/org.continuwuity.callback/user_may_invite",
        }
    };

    /// Request type for the `callback` endpoint.
    #[request]
    pub struct Request {
        /// The user ID of the user who is sending the invite.
        pub inviter: OwnedUserId,
        
        /// The user ID of the user who is being invited.
        pub invitee: OwnedUserId,
        
        /// The destination room ID to where the invite points
        pub room_id: OwnedRoomId,
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
        /// Creates a new `Request`.
        pub fn new(inviter: OwnedUserId, invitee: OwnedUserId, room_id: OwnedRoomId) -> Self {
            Self { inviter, invitee, room_id }
        }
    }

    impl Response {
        /// Creates a new `Response`.
        pub fn new() -> Self {
            Self::default()
        }
    }
}
