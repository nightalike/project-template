// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use strum::{Display, EnumIter, EnumString};

#[derive(Debug, EnumIter, Display, EnumString)]
pub enum LangName {
  Rust,
  TypeScript,
}
