use essential_types::Value;

use crate::inputs::B256;

pub struct DecVars {
    pub token: B256,
    pub new_owner: B256,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        self.token
            .to_value()
            .into_iter()
            .chain(self.new_owner.to_value())
            .map(|w| vec![w])
            .collect()
    }
}
