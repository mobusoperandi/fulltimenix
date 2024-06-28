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

#[test]
fn nixpkgs_clone_path_not_set() {
    let mut command = subject();
    command.arg("2024-06-13");
    command.env_remove("NIXPKGS_CLONE_PATH");
    command.assert().failure().stderr(predicates::str::contains(
        "`NIXPKGS_CLONE_PATH` not provided",
    ));
}

#[test]
fn nixpkgs_clone_path_io_error() {
    let mut command = subject();
    command.arg("2024-06-13");
    command.env("NIXPKGS_CLONE_PATH", "/dev/null");
    command
        .assert()
        .failure()
        .stderr(predicates::str::contains("io error:"));
}
