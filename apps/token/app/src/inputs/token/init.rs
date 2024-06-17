use crate::inputs::B256;
use essential_types::Value;

pub struct DecVars {
    pub name: B256,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        self.name.to_value().into_iter().map(|w| vec![w]).collect()
    }
}
