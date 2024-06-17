use app_utils::inputs::{Int, B256};
use essential_types::Value;

pub struct DecVars {
    pub receiver: B256,
    pub sender: B256,
    pub amount: Int,
}

impl DecVars {
    pub fn encode(&self) -> Vec<Value> {
        self.receiver
            .to_value()
            .into_iter()
            .chain(self.sender.to_value())
            .chain(self.amount.to_value())
            .map(|w| vec![w])
            .collect()
    }
}
