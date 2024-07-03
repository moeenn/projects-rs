use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "java-gradle-build-gradle.txt")]
struct BuildGradle {
    name: String,
}

#[derive(Template)]
#[template(path = "java-gradle-readme-md.txt")]
struct Readme {
    name: String,
}

#[derive(Template)]
#[template(path = "java-gradle-main-java.txt")]
struct MainJava {
    name: String,
}

#[derive(Template)]
#[template(path = "java-gradle-main-test-java.txt")]
struct MainTestJava {
    name: String,
}

pub fn new_config(name: &String) -> TemplateConfig {
    let main_src = PathBuf::new()
        .join("src")
        .join("main")
        .join("java")
        .join(&name);
    let test_src = PathBuf::new()
        .join("src")
        .join("test")
        .join("java")
        .join(&name);

    TemplateConfig {
        directories: vec![main_src.clone(), test_src.clone()],
        gitignore: vec![
            String::from(".gradle"),
            String::from("gradle"),
            String::from("gradlew"),
            String::from("gradlew.bat"),
            String::from(".vscode"),
            String::from(".idea"),
            String::from("bin"),
            String::from("build"),
            String::from("*.class"),
            String::from(".DS_Store"),
        ],
        files: vec![
            TemplateFile {
                path: PathBuf::new().join("README.md"),
                template: Box::new(Readme { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join("build.gradle"),
                template: Box::new(BuildGradle { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("Main.java"),
                template: Box::new(MainJava { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join(&test_src).join("MainTest.java"),
                template: Box::new(MainTestJava { name: name.clone() }),
            },
        ],
    }
}
