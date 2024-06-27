// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::str::FromStr;

use dialoguer::theme::ColorfulTheme;
use strum::IntoEnumIterator;

use crate::LicenseKind;

pub use self::code::{CodeProject, CodeProjectKind};

use super::{bool, input, select};

mod code;

#[derive(Debug)]
pub struct Project {
  pub name: String,
  pub description: String,
  pub kind: ProjectKind,
  pub license: Option<LicenseKind>,
  pub allow_contributions: bool,
}

#[derive(Debug)]
pub enum ProjectKind {
  Code(CodeProject),
  Other,
}

impl Project {
  pub fn get(theme: &ColorfulTheme) -> Self {
    let name = Self::name(theme);
    let description = Self::description(theme);
    let kind = Self::kind(theme);
    let license = Self::license(theme);
    let allow_contributions = Self::allow_contributions(theme);

    Self {
      name,
      description,
      kind,
      license,
      allow_contributions,
    }
  }

  fn name(theme: &ColorfulTheme) -> String {
    input(theme, "What is the name of the project?")
  }

  fn description(theme: &ColorfulTheme) -> String {
    input(theme, "What is the description of the project?")
  }

  fn kind(theme: &ColorfulTheme) -> ProjectKind {
    let options = ["Code", "Other"].map(|a| a.to_string()).to_vec();
    let selection = select(theme, "What kind of project is this?", &options, 0);

    match selection {
      0 => ProjectKind::Code(CodeProject::get(theme)),
      1 => ProjectKind::Other,
      _ => panic!(),
    }
  }

  fn license(theme: &ColorfulTheme) -> Option<LicenseKind> {
    let options = LicenseKind::iter().map(|a| a.to_str()).collect::<Vec<_>>();
    let selection = select(
      theme,
      "What kind of license does this project have?",
      &options,
      0,
    );

    LicenseKind::from_str(&options[selection]).ok()
  }

  fn allow_contributions(theme: &ColorfulTheme) -> bool {
    bool(theme, "Can this project be contributed to?", true)
  }
}
