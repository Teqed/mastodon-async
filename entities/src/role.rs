//! module containing information about a role.

use crate::RoleId;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

/// A struct containing info about a role.
/// https://docs.joinmastodon.org/entities/Role/
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Role {
    /// The ID of the role.
    pub id: RoleId,
    /// The name of the role.
    pub name: String,
    /// The hex code of the role, if there's one assigned to it.
    #[serde_as(as = "NoneAsEmptyString")]
    pub color: Option<String>,
    /// An index for the role’s position. The higher the position, the more priority the role has over other roles.
    /// The owner role does not have a position.
    pub position: Option<i32>,
    /// A bitmask that represents the sum of all permissions granted to the role.
    #[serde(with = "serialize_permissions")]
    pub permissions: Permissions,
    /// Whether the role is publicly visible as a badge on user profiles.
    pub highlighted: bool,
}

bitflags! {
    /// https://docs.joinmastodon.org/entities/Role/#permission-flags
    /// Stored as a PostgreSQL `bigint(8)` which is an `i64`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Permissions: i64 {
        const Administrator = 0x1;
        const Devops = 0x2;
        const ViewAuditLog = 0x4;
        const ViewDashboard = 0x8;
        const ManageReports = 0x10;
        const ManageFederation = 0x20;
        const ManageSettings = 0x40;
        const ManageBlocks = 0x80;
        const ManageTaxonomies = 0x100;
        const ManageAppeals = 0x200;
        const ManageUsers = 0x400;
        const ManageInvites = 0x800;
        const ManageRules = 0x1000;
        const ManageAnnouncements = 0x2000;
        const ManageCustomEmojis = 0x4000;
        const ManageWebhooks = 0x8000;
        const InviteUsers = 0x10000;
        const ManageRoles = 0x20000;
        const ManageUserAccess = 0x40000;
        const DeleteUserData = 0x80000;
    }
}

mod serialize_permissions {
    use super::Permissions;
    use serde::de::{Error, Visitor};
    use serde::{Deserializer, Serializer};
    use std::fmt::Formatter;
    use std::str::FromStr;

    /// Serialize a [`Permissions`] as an i64.
    pub fn serialize<S: Serializer>(
        permissions: &Permissions,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(permissions.bits())
    }

    struct PermissionsVisitor;

    impl<'de> Visitor<'de> for PermissionsVisitor {
        type Value = Permissions;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("an i64 representing role permissions as a flag field")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Permissions::from_bits_retain(v))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            i64::try_from(v)
                .map(|i| Permissions::from_bits_retain(i))
                .map_err(|_| E::custom(format!("out of range for i64: {}", v)))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            i64::from_str(v)
                .map(|i| Permissions::from_bits_retain(i))
                .map_err(|_| E::custom(format!("couldn't parse string as i64: {}", v)))
        }
    }

    /// Deserialize a [`Permissions`] from an i64, u64, or numeric string.
    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Permissions, D::Error> {
        deserializer.deserialize_any(PermissionsVisitor {})
    }
}
