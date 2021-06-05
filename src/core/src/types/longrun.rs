use serde::{
    Deserialize,
    Serialize,
};

use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Longrun {
    pub name: String,
    pub run: Script,
    pub finish: Option<Script>,
    pub options: ServiceOptions,
}
