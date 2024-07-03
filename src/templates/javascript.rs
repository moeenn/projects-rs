use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "js-index-mjs.txt")]
struct Index {}

#[derive(Template)]
#[template(path = "js-index-test-mjs.txt")]
struct IndexTest {}

#[derive(Template)]
#[template(path = "js-jsconfig-json.txt")]
struct JsconfigJSON {}

#[derive(Template)]
#[template(path = "js-package-json.txt")]
struct PackageJSON {
    name: String,
}

#[derive(Template)]
#[template(path = "js-readme-md.txt")]
struct Readme {
    name: String,
}

pub fn new_config(name: &String) -> TemplateConfig {
    let main_src = PathBuf::new().join("src");

    TemplateConfig {
        directories: vec![main_src.clone()],
        gitignore: vec![String::from("node_modules"), String::from(".DS_Store")],
        files: vec![
            TemplateFile {
                path: PathBuf::new().join("README.md"),
                template: Box::new(Readme { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join("package.json"),
                template: Box::new(PackageJSON { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join("jsconfig.json"),
                template: Box::new(JsconfigJSON {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("index.mjs"),
                template: Box::new(Index {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("index.test.mjs"),
                template: Box::new(IndexTest {}),
            },
        ],
    }
}
