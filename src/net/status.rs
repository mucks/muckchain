use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub height: u32,
}

#[typetag::serde]
impl Encodable for Status {}
