mod assemble;
mod cpu;
mod execute;
mod instruction;
mod optimise;
mod optimal;

use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "super-optimiser", about = "optimises small assembly")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Execute(execute::Execute),
    Optimal(optimal::Optimal),
}

impl Command {
    fn run(&self) {
        match self {
            Command::Execute(execute) => execute.run(),
            Command::Optimal(optimal) => optimal.run(),
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    opt.cmd.run();
}