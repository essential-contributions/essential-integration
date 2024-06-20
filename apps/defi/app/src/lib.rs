pub mod inputs;

#[derive(Clone, Debug)]
pub struct Swap {
    pub signed: inputs::SignedSwap,
    pub swap: inputs::SwapArgs,
}
