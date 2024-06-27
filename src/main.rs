mod render;
mod templates;

use clap::Parser;
use render::{TemplateConfig, TemplateExecutor, TemplateType};
use std::process;
use templates::java_gradle;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("java-gradle") )]
    template: String,

    #[arg(short, long, default_value_t = String::from("sandbox") )]
    name: String,

    #[arg(short, long)]
    list: bool
}

const VALID_TEMPLATES: [&str; 6] = [
    "c", "cpp-cmake", "javascript (or 'js')", "typescript (or 'ts')", "java-gradle", "python",
];

fn print_valid_templates() {
    println!("Valid templates include:");
    for template_name in VALID_TEMPLATES {
        println!(" - {}", template_name);
    }
}

fn main() {
    let args = Args::parse();
    if args.list {
        return print_valid_templates();
    }

    let template_type = match TemplateType::from_string(&args.template) {
        Some(t) => t,
        None => {
            return print_valid_templates();
        }
    };
    
    let config: TemplateConfig = match template_type {
        TemplateType::C => java_gradle::new_java_config(&args.name), // TODO: implement actual
        TemplateType::CppCmake => java_gradle::new_java_config(&args.name), // TODO: implement actual
        TemplateType::Javascript => java_gradle::new_java_config(&args.name), // TODO: implement actual
        TemplateType::Typescript => java_gradle::new_java_config(&args.name), // TODO: implement actual
        TemplateType::JavaGradle => java_gradle::new_java_config(&args.name),
        TemplateType::Python => java_gradle::new_java_config(&args.name), // TODO: implement actual
    };
    
    let executor = match TemplateExecutor::new(args.name, config) {
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
