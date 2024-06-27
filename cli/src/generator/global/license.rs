// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::LicenseKind;

pub fn add(license: &Option<LicenseKind>) -> Option<(String, String)> {
  let file = match license {
    Some(license) => match license {
      LicenseKind::Mit => Some(include_str!("../../../../files/global/LICENSE/MIT")),
      LicenseKind::Apache2_0 => Some(include_str!("../../../../files/global/LICENSE/Apache-2.0")),
      LicenseKind::Gpl2_0 => Some(include_str!(
        "../../../../files/global/LICENSE/GPL-2.0-only"
      )),
      LicenseKind::Gpl3_0 => Some(include_str!(
        "../../../../files/global/LICENSE/GPL-3.0-only"
      )),
      LicenseKind::Cc0 => Some(include_str!("../../../../files/global/LICENSE/CC0-1.0")),
    },
    None => None,
  };

  match file {
    Some(file) => Some(("LICENSE".to_string(), file.to_string())),
    None => None,
  }
}
