//! Types for invite filtering ([MSC4155]).
//! 
//! MSC4155: https://github.com/matrix-org/matrix-spec-proposals/pull/4155

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.invite_permission_config", kind = GlobalAccountData)]
pub struct InvitePermissionConfigEventContent {
    /// A list of globs matching users which are allowed to send an invite.
    /// Entries in this list supersede entries in the ignored and blocked lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_users: Option<Vec<String>>,
    /// A list of globs matching users whose invites should be ignored (as defined in [MSC4283]).
    /// 
    /// MSC4283: https://github.com/matrix-org/matrix-spec-proposals/pull/4283
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_users: Option<Vec<String>>,
    /// A list of globs matching users whose invites should be blocked (as defined in [MSC4283]).
    /// Invites from blocked users should be refused with the M_INVITE_BLOCKED status code.
    /// 
    /// MSC4283: https://github.com/matrix-org/matrix-spec-proposals/pull/4283
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_users: Option<Vec<String>>,


    /// A list of globs matching servers which are allowed to send an invite.
    /// Entries in this list supersede entries in the ignored and blocked lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_servers: Option<Vec<String>>,
    /// A list of globs matching servers whose invites should be ignored (as defined in [MSC4283]).
    /// 
    /// MSC4283: https://github.com/matrix-org/matrix-spec-proposals/pull/4283
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_servers: Option<Vec<String>>,
    /// A list of globs matching servers whose invites should be blocked (as defined in [MSC4283]).
    /// Invites from blocked servers should be refused with the M_INVITE_BLOCKED status code.
    /// 
    /// MSC4283: https://github.com/matrix-org/matrix-spec-proposals/pull/4283
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_servers: Option<Vec<String>>,
}

impl InvitePermissionConfigEventContent {
    /// Creates a new `InvitePermissionConfigEventContent` from six lists of globs.
    pub fn new(
        allowed_users: Option<Vec<String>>,
        ignored_users: Option<Vec<String>>,
        blocked_users: Option<Vec<String>>,
        allowed_servers: Option<Vec<String>>,
        ignored_servers: Option<Vec<String>>,
        blocked_servers: Option<Vec<String>>,
    ) -> Self {
        Self {
            allowed_users,
            ignored_users,
            blocked_users,
            allowed_servers,
            ignored_servers,
            blocked_servers,
        }
    }
}