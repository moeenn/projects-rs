use askama::DynTemplate;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub enum TemplateType {
    C,
    CppCmake,
    Javascript,
    Typescript,
    JavaGradle,
    Python,
}

impl TemplateType {
    pub fn from_string<'a>(value: &'a str) -> Option<Self> {
        match value {
            "c" => Some(Self::C),
            "cpp-cmake" => Some(Self::CppCmake),
            "javascript" | "js" => Some(Self::Javascript),
            "typescript" | "ts" => Some(Self::Typescript),
            "java-gradle" => Some(Self::JavaGradle),
            "python" => Some(Self::Python),
            _ => None
        }
    }
}

pub struct TemplateFile {
    pub path: PathBuf,
    pub template: Box<dyn DynTemplate>,
}

pub struct TemplateConfig {
    pub directories: Vec<PathBuf>,
    pub gitignore: Vec<String>,
    pub files: Vec<TemplateFile>,
}

pub struct TemplateExecutor {
    pub root: PathBuf,
    pub config: TemplateConfig,
}

impl TemplateExecutor {
    pub fn new(project_name: String, config: TemplateConfig) -> Result<Self, String> {
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                return Err(format!(
                    "failed to detect current directory. Details: {}",
                    e.to_string()
                ))
            }
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
            let err = match fs::create_dir_all(&dir) {
                Ok(_) => continue,
                Err(err) => err,
            };

            return Err(format!(
                "failed to created directory '{:?}'. Details: {}",
                dir,
                err.to_string()
            ));
        }

        Ok(())
    }

    fn create_gitignore(&self) -> Result<(), String> {
        let gitignore_path = self.root.join(".gitignore");

        let mut file = match File::create(gitignore_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(format!(
                    "failed to create .gitignore file. Details: {}",
                    e.to_string()
                ))
            }
        };

        for line in self.config.gitignore.iter() {
            match file.write(format!("{}\n", line).as_bytes()) {
                Ok(_) => continue,
                Err(e) => {
                    return Err(format!(
                        "failed to write to .gitignore file. Details: {}",
                        e.to_string()
                    ))
                }
            }
        }

        Ok(())
    }

    fn render_file_templates(&self) -> Result<(), String> {
        for file in self.config.files.iter() {
            let output_path = self.root.join(file.path.clone());
            let mut output_file = match File::create(&output_path) {
                Ok(f) => f,
                Err(e) => {
                    return Err(format!(
                        "failed to create output file '{:?}'. Details: {}",
                        output_path,
                        e.to_string()
                    ))
                }
            };

            let content = match file.template.dyn_render() {
                Ok(c) => c,
                Err(e) => {
                    return Err(format!(
                        "failed to render template '{:?}'. Details: {}",
                        output_path,
                        e.to_string()
                    ))
                }
            };

            if let Err(e) = output_file.write(content.as_bytes()) {
                return Err(format!(
                    "failed to write to output file '{:?}'. Details: {}",
                    output_path,
                    e.to_string()
                ));
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
