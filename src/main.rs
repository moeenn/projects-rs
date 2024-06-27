mod render;
mod templates;

use std::process;
use clap::Parser;
use render::TemplateExecutor;
use templates::java_gradle;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    template: String,

    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    let java_config = java_gradle::new_java_config(&args.name);
    let executor = match TemplateExecutor::new(args.name, java_config) {
        Ok(ex) => ex,
        Err(e) => {
            println!("error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = executor.execute() {
        println!("error: {}", e);
        process::exit(1);
    }

    println!("Project initialized successfully!");
}
