// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use clap::{Arg, ArgAction, Command};

pub use self::form::Form;
pub use self::generator::Generator;
pub use self::lang::LangName;
pub use self::license::LicenseKind;

mod form;
mod generator;
mod lang;
mod license;

fn main() {
  let clap = Command::new("Project Template Generator")
    .arg(
      Arg::new("open_repo")
        .long("repo")
        .short('r')
        .action(ArgAction::SetTrue)
        .help("Open the repository that contains the files"),
    )
    .get_matches();

  if *clap.get_one::<bool>("open_repo").unwrap_or(&false) {
    open::that("https://github.com/toolbisoftware/project-template/tree/main/files").unwrap();
    std::process::exit(0);
  }

  let form = Form::init();

  Generator::init(form);
}
