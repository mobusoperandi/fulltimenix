pub fn subject() -> assert_cmd::Command {
    let mut command = assert_cmd::Command::cargo_bin("nixpkgs-news").unwrap();
    command.env("NIXPKGS_CLONE_PATH", env!("FLAKE_PROVIDED_NIXPKGS_CLONE_PATH"));
    command
}
