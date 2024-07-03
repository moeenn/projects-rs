use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "go-main-go.txt")]
struct Main {}


#[derive(Template)]
#[template(path = "go-go-mod.txt")]
struct GoMod {
  name: String
}

pub fn new_config(name: &String) -> TemplateConfig {
  TemplateConfig {
      directories: vec![],
      gitignore: vec![name.clone()],
      files: vec![
          TemplateFile {
              path: PathBuf::new().join("go.mod"),
              template: Box::new(GoMod { name: name.clone() }),
          },
          TemplateFile {
              path: PathBuf::new().join("main.go"),
              template: Box::new(Main {}),
          },
      ],
  }
}

