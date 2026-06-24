use clap::Args;

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("CompletionArg")
            .required(false)
            .multiple(false)
            .args(["bash", "elvish", "fish", "power_shell", "zsh"])
    )
)]
pub struct CompletionArgs {
    /// Generate completion for bash
    #[arg(long, group = "CompletionArg")]
    pub bash: bool,
    /// Generate completion for elvish
    #[arg(long, group = "CompletionArg")]
    pub elvish: bool,
    /// Generate completion for fish
    #[arg(long, group = "CompletionArg")]
    pub fish: bool,
    /// Generate completion for PowerShell
    #[arg(long, group = "CompletionArg")]
    pub power_shell: bool,
    /// Generate completion for zsh
    #[arg(long, group = "CompletionArg")]
    pub zsh: bool,
}

impl CompletionArgs {
    pub fn get_bin_name<'a>(&self, default: &'a str) -> &'a str {
        if self.bash {
            "bash"
        } else if self.elvish {
            "elvish"
        } else if self.fish {
            "fish"
        } else if self.power_shell {
            if cfg!(windows) { "powershell" } else { "pwsh" }
        } else if self.zsh {
            "zsh"
        } else {
            default
        }
    }
}
