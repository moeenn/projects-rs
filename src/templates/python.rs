use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "py-app-py.txt")]
struct AppPy {}

#[derive(Template)]
#[template(path = "py-app-test-py.txt")]
struct AppTestPy {}

#[derive(Template)]
#[template(path = "py-init-py.txt")]
struct InitPy {}

#[derive(Template)]
#[template(path = "py-main-py.txt")]
struct MainPy {}

#[derive(Template)]
#[template(path = "py-readme-md.txt")]
struct Readme {
    name: String,
}

#[derive(Template)]
#[template(path = "py-requirements-txt.txt")]
struct RequirementsTxt {}

#[derive(Template)]
#[template(path = "py-tasks-py.txt")]
struct TasksPy {}

pub fn new_config(name: &String) -> TemplateConfig {
    let main_src = PathBuf::new().join("app");

    TemplateConfig {
        directories: vec![main_src.clone()],
        gitignore: vec![
            String::from(".venv"),
            String::from("*.pyc"),
            String::from("__pycache__"),
            String::from(".vscode"),
            String::from("build/"),
            String::from("*.egg-info"),
            String::from(".*_cache"),
            String::from("dist/"),
            String::from(".DS_Store"),
        ],
        files: vec![
            TemplateFile {
                path: PathBuf::new().join("tasks.py"),
                template: Box::new(TasksPy {}),
            },
            TemplateFile {
                path: PathBuf::new().join("requirements.txt"),
                template: Box::new(RequirementsTxt {}),
            },
            TemplateFile {
                path: PathBuf::new().join("README.md"),
                template: Box::new(Readme { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join("main.py"),
                template: Box::new(MainPy {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("__init__.py"),
                template: Box::new(InitPy {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("app.py"),
                template: Box::new(AppPy {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("app_test.py"),
                template: Box::new(AppTestPy {}),
            },
        ],
    }
}
