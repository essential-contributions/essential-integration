use app_utils::inputs::{Int, B256};
use essential_types::Value;

pub struct DecVars {
    pub owner: B256,
    pub amount: Int,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        self.owner
            .to_value()
            .into_iter()
            .chain(self.amount.to_value())
            .map(|w| vec![w])
            .collect()
    }
}
