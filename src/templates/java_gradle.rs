use crate::render::{TemplateConfig, TemplateFile};
use askama::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "java-gradle-build-gradle.txt")]
struct JavaGradleBuildGradle {
    name: String,
}

#[derive(Template)]
#[template(path = "java-gradle-readme-md.txt")]
struct JavaGradleReadmeMd {
    name: String,
}

#[derive(Template)]
#[template(path = "java-gradle-main-java.txt")]
struct JavaGradleMainJava {
    name: String,
}

#[derive(Template)]
#[template(path = "java-gradle-main-test-java.txt")]
struct JavaGradleMainTestJava {
    name: String,
}

pub fn new_java_config(name: &String) -> TemplateConfig {
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
            String::from(".vscode"),
            String::from("bin"),
            String::from("build"),
            String::from("*.class"),
            String::from(".DS_Store"),
        ],
        files: vec![
            TemplateFile {
                path: PathBuf::new().join("README.md"),
                template: Box::new(JavaGradleReadmeMd { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join("build.gradle"),
                template: Box::new(JavaGradleBuildGradle { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join(&main_src).join("Main.java"),
                template: Box::new(JavaGradleMainJava { name: name.clone() }),
            },
            TemplateFile {
                path: PathBuf::new().join(&test_src).join("MainTest.java"),
                template: Box::new(JavaGradleMainTestJava { name: name.clone() }),
            },
        ],
    }
}
