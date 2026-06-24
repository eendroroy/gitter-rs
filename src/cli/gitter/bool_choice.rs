use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq, Default)]
pub enum BoolChoice {
    #[default]
    Always,
    Never,
}
