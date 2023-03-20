//! A module containing everything relating to an account as seen by the admin API.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use time::{serde::iso8601, OffsetDateTime};

/// A struct representing an Account as seen by the admin API.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Account {
    /// The ID of the account.
    pub id: AccountId,
    /// The username of the account.
    pub username: String,
    /// The domain of the account. Will be `None` for local accounts.
    pub domain: Option<String>,
    /// When the account was first discovered.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The email address associated with the account.
    pub email: String,
    /// The IP address last used to login to this account.
    /// TODO: read this as an [`Ip`] struct from Mastodon 3.5 instances:
    ///     https://docs.joinmastodon.org/entities/Admin_Account/#ip
    pub ip: Option<IpAddr>,
    /// All known IP addresses associated with this account.
    pub ips: Vec<Ip>,
    /// The locale of the account as a [BCP 47](https://www.rfc-editor.org/info/bcp47) language tag.
    pub locale: String,
    /// The reason given when requesting an invite, if applicable to this instance.
    pub invite_request: Option<String>,
    /// The current role of the account.
    /// TODO: read this as a legacy role string from Mastodon pre-4.0 instances:
    ///     https://docs.joinmastodon.org/entities/Admin_Account/#role
    pub role: Role,
    /// Whether the account has confirmed their email address.
    pub confirmed: bool,
    /// Whether the account is currently approved.
    pub approved: bool,
    /// Whether the account is currently disabled.
    pub disabled: bool,
    /// Whether the account is currently silenced.
    pub silenced: bool,
    /// Whether the account is currently suspended.
    pub suspended: bool,
    /// User-level information about the account.
    pub account: crate::prelude::Account,
    /// The ID of the [`Application`] that created this account, if applicable.
    pub created_by_application_id: Option<ApplicationId>,
    /// The ID of the [`crate::prelude::Account`] that invited this user, if applicable.
    pub invited_by_account_id: Option<AccountId>,
}
