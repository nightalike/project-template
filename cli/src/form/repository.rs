// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::str::FromStr;

use dialoguer::{theme::ColorfulTheme, Input};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use super::{input, select};

#[derive(Debug)]
pub struct Repository {
  pub web: RepositoryWebName,
  pub owner: String,
  pub name: String,
  pub main_branch: String,
}

#[derive(Debug, EnumIter, Display, EnumString)]
pub enum RepositoryWebName {
  GitHub,
  #[strum(to_string = "Toolbi Git")]
  Toolbi,
}

impl Repository {
  pub fn get(theme: &ColorfulTheme) -> Self {
    let web = Self::web(theme);
    let owner = Self::owner(theme);
    let name = Self::name(theme);
    let main_branch = Self::main_branch(theme);

    Self {
      web,
      owner,
      name,
      main_branch,
    }
  }

  fn web(theme: &ColorfulTheme) -> RepositoryWebName {
    let options = RepositoryWebName::iter()
      .map(|a| a.to_string())
      .collect::<Vec<_>>();
    let selection = select(theme, "Where is the repository hosted?", &options, 0);

    RepositoryWebName::from_str(&options[selection]).unwrap()
  }

  fn owner(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
      .with_prompt("Who is the owner of the repository?")
      .with_initial_text("toolbisoftware")
      .interact_text()
      .unwrap()
  }

  fn name(theme: &ColorfulTheme) -> String {
    input(theme, "What is the name of the repository?")
  }

  fn main_branch(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
      .with_prompt("How is the main branch called?")
      .with_initial_text("main")
      .interact_text()
      .unwrap()
  }
}
