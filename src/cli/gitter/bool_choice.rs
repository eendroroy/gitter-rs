use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum BoolChoice {
    Always,
    Never,
}
