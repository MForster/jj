// Copyright 2022 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use itertools::Itertools;
use regex::Regex;

use crate::common::TestEnvironment;

pub mod common;

#[test]
fn test_config_list_single() {
    let test_env = TestEnvironment::default();
    test_env.add_config(
        r###"
    [test-table]
    somekey = "some value"
    "###
        .as_bytes(),
    );

    let stdout = test_env.jj_cmd_success(
        test_env.env_root(),
        &["config", "list", "test-table.somekey"],
    );
    insta::assert_snapshot!(stdout, @r###"
    test-table.somekey="some value"
    "###);
}

#[test]
fn test_config_list_table() {
    let test_env = TestEnvironment::default();
    test_env.add_config(
        r###"
    [test-table]
    x = true
    y.foo = "abc"
    y.bar = 123
    "###
        .as_bytes(),
    );
    let stdout = test_env.jj_cmd_success(test_env.env_root(), &["config", "list", "test-table"]);
    insta::assert_snapshot!(
        stdout,
        @r###"
    test-table.x=true
    test-table.y.bar=123
    test-table.y.foo="abc"
    "###);
}

#[test]
fn test_config_list_array() {
    let test_env = TestEnvironment::default();
    test_env.add_config(
        r###"
    test-array = [1, "b", 3.4]
    "###
        .as_bytes(),
    );
    let stdout = test_env.jj_cmd_success(test_env.env_root(), &["config", "list", "test-array"]);
    insta::assert_snapshot!(stdout, @r###"
    test-array=[1, "b", 3.4]
    "###);
}

#[test]
fn test_config_list_inline_table() {
    let test_env = TestEnvironment::default();
    test_env.add_config(
        r###"
        [[test-table]]
        x = 1
        [[test-table]]
        y = ["z"]
    "###
        .as_bytes(),
    );
    let stdout = test_env.jj_cmd_success(test_env.env_root(), &["config", "list", "test-table"]);
    insta::assert_snapshot!(stdout, @r###"
    test-table=[{x=1}, {y=["z"]}]
    "###);
}

#[test]
fn test_config_list_all() {
    let test_env = TestEnvironment::default();
    test_env.add_config(
        r###"
    test-val = [1, 2, 3]
    [test-table]
    x = true
    y.foo = "abc"
    y.bar = 123
    "###
        .as_bytes(),
    );
    let stdout = test_env.jj_cmd_success(test_env.env_root(), &["config", "list"]);
    insta::assert_snapshot!(
        find_stdout_lines(r"(test-val|test-table\b[^=]*)", &stdout),
        @r###"
    test-table.x=true
    test-table.y.bar=123
    test-table.y.foo="abc"
    test-val=[1, 2, 3]
    "###);
}

#[test]
fn test_config_layer_override_default() {
    let test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "repo", "--git"]);
    let repo_path = test_env.env_root().join("repo");
    let config_key = "merge-tools.vimdiff.program";

    // Default
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    merge-tools.vimdiff.program="vim"
    "###);

    // User
    test_env.add_config(format!("{config_key} = {value:?}\n", value = "user").as_bytes());
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    merge-tools.vimdiff.program="user"
    "###);

    // Repo
    std::fs::write(
        repo_path.join(".jj/repo/config.toml"),
        format!("{config_key} = {value:?}\n", value = "repo"),
    )
    .unwrap();
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    merge-tools.vimdiff.program="repo"
    "###);

    // Command argument
    let stdout = test_env.jj_cmd_success(
        &repo_path,
        &[
            "config",
            "list",
            config_key,
            "--config-toml",
            &format!("{config_key}={value:?}", value = "command-arg"),
        ],
    );
    insta::assert_snapshot!(stdout, @r###"
    merge-tools.vimdiff.program="command-arg"
    "###);
}

#[test]
fn test_config_layer_override_env() {
    let mut test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "repo", "--git"]);
    let repo_path = test_env.env_root().join("repo");
    let config_key = "ui.editor";

    // Environment base
    test_env.add_env_var("EDITOR", "env-base");
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="env-base"
    "###);

    // User
    test_env.add_config(format!("{config_key} = {value:?}\n", value = "user").as_bytes());
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="user"
    "###);

    // Repo
    std::fs::write(
        repo_path.join(".jj/repo/config.toml"),
        format!("{config_key} = {value:?}\n", value = "repo"),
    )
    .unwrap();
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="repo"
    "###);

    // Environment override
    test_env.add_env_var("JJ_EDITOR", "env-override");
    let stdout = test_env.jj_cmd_success(&repo_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="env-override"
    "###);

    // Command argument
    let stdout = test_env.jj_cmd_success(
        &repo_path,
        &[
            "config",
            "list",
            config_key,
            "--config-toml",
            &format!("{config_key}={value:?}", value = "command-arg"),
        ],
    );
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="command-arg"
    "###);
}

#[test]
fn test_config_layer_workspace() {
    let test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "--git", "main"]);
    let main_path = test_env.env_root().join("main");
    let secondary_path = test_env.env_root().join("secondary");
    let config_key = "ui.editor";

    std::fs::write(main_path.join("file"), "contents").unwrap();
    test_env.jj_cmd_success(&main_path, &["new"]);
    test_env.jj_cmd_success(
        &main_path,
        &["workspace", "add", "--name", "second", "../secondary"],
    );

    // Repo
    std::fs::write(
        main_path.join(".jj/repo/config.toml"),
        format!("{config_key} = {value:?}\n", value = "main-repo"),
    )
    .unwrap();
    let stdout = test_env.jj_cmd_success(&main_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="main-repo"
    "###);
    let stdout = test_env.jj_cmd_success(&secondary_path, &["config", "list", config_key]);
    insta::assert_snapshot!(stdout, @r###"
    ui.editor="main-repo"
    "###);
}

#[test]
fn test_config_edit_missing_opt() {
    let test_env = TestEnvironment::default();
    let stderr = test_env.jj_cmd_cli_error(test_env.env_root(), &["config", "edit"]);
    insta::assert_snapshot!(stderr, @r###"
    error: The following required arguments were not provided:
      <--user|--repo>

    Usage: jj config edit <--user|--repo>

    For more information try '--help'
    "###);
}

#[test]
fn test_config_edit_user() {
    let mut test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "repo", "--git"]);
    let repo_path = test_env.env_root().join("repo");
    let edit_script = test_env.set_up_fake_editor();

    std::fs::write(
        edit_script,
        format!("expectpath\n{}", test_env.config_dir().to_str().unwrap()),
    )
    .unwrap();
    test_env.jj_cmd_success(&repo_path, &["config", "edit", "--user"]);
}

#[test]
fn test_config_edit_repo() {
    let mut test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "repo", "--git"]);
    let repo_path = test_env.env_root().join("repo");
    let edit_script = test_env.set_up_fake_editor();

    std::fs::write(
        edit_script,
        format!(
            "expectpath\n{}",
            repo_path.join(".jj/repo/config.toml").to_str().unwrap()
        ),
    )
    .unwrap();
    test_env.jj_cmd_success(&repo_path, &["config", "edit", "--repo"]);
}

fn find_stdout_lines(keyname_pattern: &str, stdout: &str) -> String {
    let key_line_re = Regex::new(&format!(r"(?m)^{keyname_pattern}=.*$")).unwrap();
    key_line_re
        .find_iter(stdout)
        .map(|m| m.as_str())
        .collect_vec()
        .join("\n")
}
