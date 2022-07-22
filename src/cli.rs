use clap::{Parser, Subcommand};

pub fn init() -> Cli {
    Cli::parse()
}

#[derive(Debug, Parser)]
#[clap(name = "mococp")]
#[clap(about = "Moco CLI", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Moco login", long_about = None)]
    Login {},
    #[clap(about = "Performance report summary", long_about = None)]
    Report {},
    #[clap(about = "List activities", long_about = None)]
    List {
        #[clap(long)]
        today: bool,

        #[clap(long)]
        week: bool,

        #[clap(long)]
        month: bool,
    },
    #[clap(about = "Create new activity", long_about = None)]
    New {
        #[clap(long)]
        project: Option<i64>,

        #[clap(long)]
        task: Option<i64>,

        #[clap(long)]
        date: Option<String>,

        #[clap(long)]
        hours: Option<f64>,

        #[clap(long)]
        description: Option<String>,
    },
    #[clap(about = "Edit activity", long_about = None)]
    Edit {
        #[clap(long)]
        activity: Option<i64>,

        #[clap(long)]
        project: i64,

        #[clap(long)]
        task: i64,

        #[clap(long)]
        date: Option<String>,

        #[clap(long)]
        hours: Option<f64>,

        #[clap(long)]
        description: Option<String>,
    },
    #[clap(about = "Delete activity", long_about = None)]
    Rm {
        #[clap(long)]
        activity: i64,
    },
    #[clap(about = "Start/Stop activity timer", long_about = None)]
    Timer {
        #[clap(subcommand)]
        system: Timer,
    },
    #[clap(about = "List customers", long_about = None)]
    Customers {},
    #[clap(about = "List projects", long_about = None)]
    Projects {
        #[clap(long)]
        customer: Option<i64>,
    },
    #[clap(about = "List project tasks", long_about = None)]
    Tasks {
        #[clap(long)]
        project: i64,
    },
    #[clap(about = "List activities", long_about = None)]
    Activities {
        #[clap(long)]
        from: String,

        #[clap(long)]
        to: String,
    },
    #[clap(about = "Get activity", long_about = None)]
    Activity {
        #[clap(long)]
        activity: i64,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum Timer {
    Start {
        #[clap(long)]
        activity: Option<i64>,
    },
    Stop,
}
