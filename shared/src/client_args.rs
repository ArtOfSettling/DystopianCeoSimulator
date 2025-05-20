use crate::OperatorMode;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Client")]
#[command(about = "Live the life of a powerful CEO. This is the terminal client. Server must already be running", long_about = None)]
pub struct ClientArgs {
    #[arg(short, long, default_value = "operator")]
    pub operator_mode: OperatorModeArg,
}

#[derive(Debug, Clone)]
pub enum OperatorModeArg {
    Operator,
    DashboardViewer,
}

impl std::str::FromStr for OperatorModeArg {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "operator" => Ok(OperatorModeArg::Operator),
            "dashboard" => Ok(OperatorModeArg::DashboardViewer),
            other => panic!("{}", format!("Unknown client hello command '{}'", other)),
        }
    }
}

impl From<OperatorModeArg> for OperatorMode {
    fn from(arg: OperatorModeArg) -> Self {
        match arg {
            OperatorModeArg::Operator => OperatorMode::Operator,
            OperatorModeArg::DashboardViewer => OperatorMode::DashboardViewer,
        }
    }
}
