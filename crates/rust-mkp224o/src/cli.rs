use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rust-mkp224o",
    about = "A wrapper for mkp224o installation and execution",
    version = "0.1.0"
)]
#[clap(color = concolor_clap::color_choice())]
#[clap(group = clap::ArgGroup::new("command").required(true))]
pub struct Mkp224oCli {
    #[arg(
        trailing_var_arg = true, 
        value_name = "COMMAND", 
        num_args = 0.., 
        help="Passes the command after the \"--\" to the mkp224o executable",
        group = "command"
    )]
    pub command_args: Option<Vec<String>>,

    #[arg(
        short = 'i',
        long = "install",
        help = "Installs the newest version of mkp224o by overriding the old installation.",
        group = "command"
    )]
    pub install: bool,

    #[arg(
        short = 'u',
        long = "update",
        help = "Updates mkp224o to the newest version. Alias for -i or --install.",
        group = "command"
    )]
    pub update: bool,
}
