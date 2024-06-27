// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use dialoguer::{theme::ColorfulTheme, Input, Select};

pub use self::project::{CodeProjectKind, Project, ProjectKind};
pub use self::repository::Repository;

mod project;
mod repository;

#[derive(Debug)]
pub struct Form {
  pub project: Project,
  pub repository: Repository,
}

impl Form {
  pub fn init() -> Self {
    let theme: &ColorfulTheme = &ColorfulTheme::default();

    let project = Project::get(theme);
    let repository = Repository::get(theme);

    Self {
      project,
      repository,
    }
  }
}

pub fn input(theme: &ColorfulTheme, prompt: &str) -> String {
  Input::with_theme(theme)
    .with_prompt(prompt)
    .interact_text()
    .unwrap()
}

pub fn select(theme: &ColorfulTheme, prompt: &str, options: &Vec<String>, default: usize) -> usize {
  Select::with_theme(theme)
    .with_prompt(prompt)
    .items(&options[..])
    .default(default)
    .interact()
    .unwrap()
}

pub fn bool(theme: &ColorfulTheme, prompt: &str, default: bool) -> bool {
  let options = ["true", "false"].map(|a| a.to_string()).to_vec();
  let default = match default {
    true => 0,
    false => 1,
  };
  let selection = select(theme, prompt, &options, default);

  match selection {
    0 => true,
    1 => false,
    _ => panic!(),
  }
}
