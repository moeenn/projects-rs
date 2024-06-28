use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "cpp-cmake-cmakelists-txt.txt")]
struct CMakeLists {
    name: String,
}

#[derive(Template)]
#[template(path = "cpp-cmake-readme-md.txt")]
struct Readme {
    name: String,
}

#[derive(Template)]
#[template(path = "cpp-cmake-main-cpp.txt")]
struct Main {}

pub fn new_config(name: &String) -> TemplateConfig {
    let main_src = PathBuf::new().join("src");

    TemplateConfig {
        directories: vec![main_src.clone()],
        gitignore: vec![
            String::from("CMakeFiles/*"),
            String::from("cmake_install.cmake"),
            String::from("CMakeCache.txt"),
            String::from("Makefile"),
            String::from(".cache"),
            String::from("compile_commands.json"),
            String::from("bin"),
            String::from(".DS_Store"),
        ],
        files: vec![
            TemplateFile {
                path: PathBuf::new().join("CMakeLists.txt"),
                template: Box::new(CMakeLists { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join("README.md"),
                template: Box::new(Readme { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("main.cpp"),
                template: Box::new(Main {}),
            },
        ],
    }
}
