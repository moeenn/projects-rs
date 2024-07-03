use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "c-main-c.txt")]
struct Main {}

#[derive(Template)]
#[template(path = "c-makefile.txt")]
struct Makefile {
    name: String,
}

pub fn new_config(name: &String) -> TemplateConfig {
    let main_src = PathBuf::new().join("src");
    let bin = PathBuf::new().join("bin");

    TemplateConfig {
        directories: vec![main_src.clone(), bin],
        gitignore: vec![String::from("bin/*"), String::from("*.o")],
        files: vec![
            TemplateFile {
                path: PathBuf::new().join("Makefile"),
                template: Box::new(Makefile { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("main.c"),
                template: Box::new(Main {}),
            },
        ],
    }
}
