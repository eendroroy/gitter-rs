use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("ScriptArg")
            .required(false)
            .multiple(false)
            .args(["bash", "elvish", "fish", "power_shell", "zsh"])
    )
)]
pub struct ScriptArgs {
    /// Run script via bash
    #[arg(long, group = "ScriptArg")]
    pub bash: bool,
    /// Run script via elvish
    #[arg(long, group = "ScriptArg")]
    pub elvish: bool,
    /// Run script via fish
    #[arg(long, group = "ScriptArg")]
    pub fish: bool,
    /// Run script via PowerShell
    #[arg(long, group = "ScriptArg")]
    pub power_shell: bool,
    /// Run script via zsh
    #[arg(long, group = "ScriptArg")]
    pub zsh: bool,

    /// Path to the script
    #[arg(short = 'p', long = "path")]
    pub path: PathBuf,

    /// Process placeholders inside the script
    #[arg(short = 'p', long, action = clap::ArgAction::SetTrue)]
    pub placeholder: bool,
}

impl ScriptArgs {
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
