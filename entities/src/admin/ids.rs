use crate::define_ids;
use paste::paste;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Display, Formatter};

define_ids!(IpBlockId,);
