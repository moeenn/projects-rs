use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "ts-index-test-ts.txt")]
struct IndexTest {}

#[derive(Template)]
#[template(path = "ts-index-ts.txt")]
struct Index {}

#[derive(Template)]
#[template(path = "ts-package-json.txt")]
struct PackageJSON {
    name: String,
}

#[derive(Template)]
#[template(path = "ts-readme-md.txt")]
struct Readme {
    name: String,
}

#[derive(Template)]
#[template(path = "ts-swrrc.txt")]
struct SWRRc {}

#[derive(Template)]
#[template(path = "ts-tsconfig-json.txt")]
struct TsconfigJSON {}

pub fn new_config(name: &String) -> TemplateConfig {
    let main_src = PathBuf::new().join("src");

    TemplateConfig {
        directories: vec![main_src.clone()],
        gitignore: vec![
            String::from("node_modules"),
            String::from("build"),
            String::from(".DS_Store"),
        ],
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
                path: PathBuf::new().join(".swrrc"),
                template: Box::new(SWRRc {}),
            },
            TemplateFile {
                path: PathBuf::new().join("tsconfig.json"),
                template: Box::new(TsconfigJSON {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("index.ts"),
                template: Box::new(Index {}),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("index.test.ts"),
                template: Box::new(IndexTest {}),
            },
        ],
    }
}
