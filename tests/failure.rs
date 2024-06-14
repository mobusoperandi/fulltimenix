use indoc::indoc;
use util::subject;

mod util;

#[test]
fn date_not_friday() {
    let mut command = subject();
    command.arg("2024-06-13");
    command
        .assert()
        .failure()
        .stderr(predicates::str::contains("provided date is not a Friday"));
}
