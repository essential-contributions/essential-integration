use crate::inputs::{Int, B256};
use essential_types::Value;

pub struct DecVars {
    pub burner: B256,
    pub amount: Int,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        self.burner
            .to_value()
            .into_iter()
            .chain(self.amount.to_value())
            .map(|w| vec![w])
            .collect()
    }
}
