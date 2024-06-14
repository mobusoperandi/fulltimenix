pub fn subject() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("nixpkgs-news").unwrap()
}
