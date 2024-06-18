use app_utils::inputs::{Int, WriteDecVars, B256};

pub struct DecVars {
    pub token_name: B256,
    pub token_symbol: B256,
    pub decimals: Int,
}

impl DecVars {
    pub fn encode(&self) -> Vec<essential_types::Value> {
        let Self {
            token_name,
            token_symbol,
            decimals,
        } = self;
        let mut decision_variables = vec![];

        token_name.write_dec_var(&mut decision_variables);
        token_symbol.write_dec_var(&mut decision_variables);
        decimals.write_dec_var(&mut decision_variables);

        decision_variables
    }
}
