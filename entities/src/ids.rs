use paste::paste;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Display, Formatter};

#[macro_export]
macro_rules! define_ids {
    ($name:ident, $($rest:ident,)+) => {
        define_ids!($name,);
        static_assertions::assert_not_impl_any!(
            $name: $(PartialEq<$rest>,)+
        );
        define_ids!($($rest,)+);
    };
    ($name:ident,) => {
        paste! {
            #[doc = "Wrapper type for a(n) " $name " string."]
            #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
            #[serde(transparent)]
            pub struct $name(String);

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }

            impl $name {
                pub fn new(value: impl Into<String>) -> Self {
                    Self(value.into())
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<$name, D::Error> {
                    deserializer.deserialize_any([<$name Visitor>] {})
                }
            }

            struct [<$name Visitor>];

            impl<'de> Visitor<'de> for [<$name Visitor>] {
                type Value = $name;

                fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                    formatter.write_str("an ID as a string or integer")
                }

                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok($name::new(v.to_string()))
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok($name::new(v.to_string()))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok($name::new(v))
                }
            }
        }
    };
    () => {}
}

define_ids!(
    AccountId,
    ApplicationId,
    AttachmentId,
    FilterId,
    ListId,
    MentionId,
    NotificationId,
    SubscriptionId,
    RelationshipId,
    ReportId,
    RoleId,
    StatusId,
);
