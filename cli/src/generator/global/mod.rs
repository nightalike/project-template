// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use crate::Form;

mod default;
mod gitignore;
mod license;
mod readme;

pub fn get(form: Form) -> Vec<(String, String)> {
  let gitignore = gitignore::add(&form.project.kind);
  let license = license::add(&form.project.license);
  let default = default::add(form.project.allow_contributions, &form.project.kind);
  let readme = readme::add(
    form.project.name,
    form.project.description,
    form.repository.web,
    form.repository.owner,
    form.repository.name,
    form.repository.main_branch,
    form.project.license,
  );

  vec![gitignore, readme]
    .into_iter()
    .chain(license.into_iter())
    .chain(default.into_iter())
    .collect()
}
