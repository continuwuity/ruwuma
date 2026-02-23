//! Server discovery endpoints.

pub mod discover_homeserver;
pub mod discover_support;
#[cfg(feature = "unstable-msc2965")]
pub mod get_authentication_issuer;
#[cfg(feature = "unstable-msc2965")]
pub mod get_authorization_server_metadata;
#[cfg(feature = "unstable-msc4143")]
pub mod get_rtc_transports;
pub mod get_capabilities;
pub mod get_supported_versions;
