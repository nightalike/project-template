// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::form::ProjectKind;

pub fn add(allow_contributions: bool, project_kind: &ProjectKind) -> Vec<(String, String)> {
  let mut files = vec![
    (
      ".editorconfig".to_string(),
      include_str!("../../../../files/global/.editorconfig").to_string(),
    ),
    (
      ".prettierrc".to_string(),
      include_str!("../../../../files/global/.prettierrc").to_string(),
    ),
    (
      "CODE_OF_CONDUCT.md".to_string(),
      include_str!("../../../../files/global/CODE_OF_CONDUCT.md").to_string(),
    ),
  ];

  if allow_contributions {
    files.push((
      "CONTRIBUTING.md".to_string(),
      include_str!("../../../../files/global/CONTRIBUTING.md").to_string(),
    ));
  }

  if let ProjectKind::Code(_) = project_kind {
    files.push((
      "SECURITY.md".to_string(),
      include_str!("../../../../files/global/CONTRIBUTING.md").to_string(),
    ));
  }

  files
}
