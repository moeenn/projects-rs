use std::{fs::{self, File}, io::Write, path::PathBuf};
use std::process;
use clap::Parser;
use askama::{DynTemplate, Template};

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
    template: Box<dyn DynTemplate>,
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
    pub fn new(project_name: String, config: TemplateConfig) -> Result<Self, String> {
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

        let mut file = match File::create(gitignore_path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        for line in self.config.gitignore.iter() {
            match file.write(format!("{}\n", line).as_bytes()) {
                Ok(_) => continue,
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(())
    }

    fn render_file_templates(&self) -> Result<(), String> {
        for file in self.config.files.iter() {
            let output_path = self.root.join(file.path.clone());   
            let mut output_file = match File::create(output_path) {
                Ok(f) => f,
                Err(e) => return Err(e.to_string()),
            };

            let content = match file.template.dyn_render() {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };

            if let Err(e) = output_file.write(content.as_bytes()) {
                return Err(e.to_string());
            }
        }

        Ok(())
    }

    pub fn execute(&self) -> Result<(), String> {
        self.create_directories()?;
        self.create_gitignore()?;
        self.render_file_templates()?;
        Ok(())
    }
}

#[derive(Template)]
#[template(path = "java-gradle-build-gradle.txt")]
struct JavaGradleBuildGradle {
    name: String
}

#[derive(Template)]
#[template(path = "java-gradle-readme-md.txt")]
struct JavaGradleReadmeMd {
    name: String
}

#[derive(Template)]
#[template(path = "java-gradle-main-java.txt")]
struct JavaGradleMainJava {
    name: String
}

#[derive(Template)]
#[template(path = "java-gradle-main-test-java.txt")]
struct JavaGradleMainTestJava {
    name: String
}

fn main() {
    let args = Args::parse();

    let main_src = PathBuf::new().join("src").join("main").join("java").join(&args.name);
    let test_src = PathBuf::new().join("src").join("test").join("java").join(&args.name);

    let java_config = TemplateConfig {
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
            TemplateFile { 
                path: PathBuf::new().join("README.md"), 
                template: Box::new(JavaGradleReadmeMd { name: args.name.clone() }),
            },
            TemplateFile { 
                path: PathBuf::new().join("build.gradle"), 
                template: Box::new(JavaGradleBuildGradle { name: args.name.clone() }), 
            },
            TemplateFile { 
                path: PathBuf::new().join(&main_src).join("Main.java"),
                template: Box::new(JavaGradleMainJava { name: args.name.clone() }),
            },
            TemplateFile { 
                path: PathBuf::new().join(&test_src).join("MainTest.java"), 
                template: Box::new(JavaGradleMainTestJava { name: args.name.clone() }),
            },
        ],
    };

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
}
