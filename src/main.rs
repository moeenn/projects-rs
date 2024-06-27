use std::{fs::{self, OpenOptions}, io::Write, path::PathBuf};
use std::process;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    template: String,

    #[arg(short, long)]
    name: String,
}

struct TemplateFile {
    path: PathBuf,
    // template: String, // TODO: implement actual
}

struct TemplateConfig {
    directories: Vec<PathBuf>,
    gitignore: Vec<String>,
    files: Vec<TemplateFile>
}

struct TemplateExecutor {
    root: PathBuf,
    config: TemplateConfig,
}

impl TemplateExecutor {
    fn new(project_name: String, config: TemplateConfig) -> Result<Self, String> {
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => return Err(e.to_string()),
        };

        let executor = TemplateExecutor {
            root: current_dir.join(project_name),
            config,
        };

        Ok(executor)
    }

    fn create_directories(&self) -> Result<(), String> {
        for dir_name in self.config.directories.iter() {
            let dir = self.root.join(dir_name);

            let err = match fs::create_dir_all(dir) {
                Ok(_) => continue,
                Err(err) => err 
            };

            return Err(err.to_string());
        }

        Ok(())
    }

    fn create_gitignore(&self) -> Result<(), String> {
        let gitignore_path = self.root.join(".gitignore");
        let mut file = match OpenOptions::new()
            .append(true)
            .open(gitignore_path) {
                Ok(f) => f,
                Err(e) => return Err(e.to_string()), 
            };

        for line in self.config.gitignore.iter() {
            match file.write(line.as_bytes()) {
                Ok(_) => continue,
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    let main_src = PathBuf::new().join("src").join("main").join("java").join(&args.name);
    let test_src = PathBuf::new().join("src").join("test").join("java").join(&args.name);

    let javaConfig = TemplateConfig {
        directories: vec![main_src.clone(), test_src.clone()],
        gitignore: vec![
            String::from(".gradle"),
			String::from(".vscode"),
			String::from("bin"),
			String::from("build"),
			String::from("*.class"),
			String::from(".DS_Store"),
        ],
        files: vec![
            TemplateFile { path: PathBuf::new().join("README.md") },
            TemplateFile { path: PathBuf::new().join("build.gradle") },
            TemplateFile { path: PathBuf::new().join(&main_src).join("Main.java") },
            TemplateFile { path: PathBuf::new().join(&test_src).join("MainTest.java") },
        ],
    };

    let executor = match TemplateExecutor::new(args.name, javaConfig) {
        Ok(ex) => ex,
        Err(e) => {
            println!("error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = executor.create_directories() {
        println!("error: {}", e);
        process::exit(1);
    }

    if let Err(e) = executor.create_gitignore() {
        println!("error: {}", e);
        process::exit(1);
    }
}
