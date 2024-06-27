// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::str::FromStr;

use dialoguer::theme::ColorfulTheme;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::{
  form::{bool, select},
  LangName,
};

#[derive(Debug)]
pub struct CodeProject {
  pub lang: LangName,
  pub is_monorepo: bool,
  pub kind: CodeProjectKind,
}

#[derive(Debug, EnumIter, Display, EnumString)]
pub enum CodeProjectKind {
  App,
  Library,
}

impl CodeProject {
  pub fn get(theme: &ColorfulTheme) -> Self {
    let lang = Self::lang(theme);
    let is_monorepo = Self::is_monorepo(theme);
    let kind = Self::kind(theme);

    Self {
      lang,
      is_monorepo,
      kind,
    }
  }

  fn lang(theme: &ColorfulTheme) -> LangName {
    let options = LangName::iter().map(|a| a.to_string()).collect::<Vec<_>>();
    let selection = select(
      theme,
      "What programming language is used primarily?",
      &options,
      0,
    );

    LangName::from_str(&options[selection]).unwrap()
  }

  fn is_monorepo(theme: &ColorfulTheme) -> bool {
    bool(theme, "Is this project a monorepo?", false)
  }

  fn kind(theme: &ColorfulTheme) -> CodeProjectKind {
    let options = CodeProjectKind::iter()
      .map(|a| a.to_string())
      .collect::<Vec<_>>();
    let selection = select(theme, "What kind of code project is this?", &options, 0);

    CodeProjectKind::from_str(&options[selection]).unwrap()
  }
}
