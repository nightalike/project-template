// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::{
  form::{CodeProjectKind, ProjectKind},
  LangName,
};

pub fn add(project_kind: &ProjectKind) -> (String, String) {
  let file = match project_kind {
    ProjectKind::Code(code_project) => match code_project.kind {
      CodeProjectKind::App => match code_project.lang {
        LangName::Rust => {
          include_str!("../../../../files/global/.gitignore/.gitignore")
        }
        LangName::TypeScript => {
          include_str!("../../../../files/global/.gitignore/typescript/app/.gitignore")
        }
      },
      CodeProjectKind::Library => match code_project.lang {
        LangName::Rust => {
          include_str!("../../../../files/global/.gitignore/.gitignore")
        }
        LangName::TypeScript => {
          include_str!("../../../../files/global/.gitignore/typescript/package/.gitignore")
        }
      },
    },
    ProjectKind::Other => {
      include_str!("../../../../files/global/.gitignore/.gitignore")
    }
  };

  (".gitignore".to_string(), file.to_string())
}
