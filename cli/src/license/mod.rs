// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use strum::{EnumIter, EnumString};

#[derive(Debug, EnumIter, EnumString)]
pub enum LicenseKind {
  Mit,
  Apache2_0,
  Gpl2_0,
  Gpl3_0,
  Cc0,
}

impl LicenseKind {
  pub fn to_str(self) -> String {
    match self {
      LicenseKind::Mit => "MIT",
      LicenseKind::Apache2_0 => "Apache-2.0",
      LicenseKind::Gpl2_0 => "GPL-2.0-only",
      LicenseKind::Gpl3_0 => "GPL-3.0-only",
      LicenseKind::Cc0 => "CC0-1.0",
    }
    .to_string()
  }
}
