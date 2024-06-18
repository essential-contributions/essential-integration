use app_utils::inputs::{Int, B256};
use essential_types::Value;

pub struct DecVars {
    pub token: Int,
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
