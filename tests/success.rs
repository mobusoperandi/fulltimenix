mod util;

use indoc::indoc;
use util::subject;

#[test]
fn a() {
    let mut command = subject();
    command.arg("2024-06-14");
    //command.env("NIXPKGS_CLONE_PATH", env!("NIXPKGS_"))
    command.assert().success().stdout(indoc! {r#"
        ## Packages (10)

        1. `bottom` - @contributor - "A cross-platform graphical process/system monitor with a customizable interface"
    "#});
}
