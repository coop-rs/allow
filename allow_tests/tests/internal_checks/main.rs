use std::path::Path;

//use colored::Colorize;
use ui_test::color_eyre::Result;
use ui_test::{CommandBuilder, Config, Match, Mode, OutputConflictHandling};

fn main() -> Result<()> {
    run("incorrect_lint")?;
    Ok(())
}

fn run(sub_dir_name: &'static str) -> Result<()> {
    let internal_checks_dir = Path::new(file!()).parent().unwrap();
    let internal_checks_sub_dir = internal_checks_dir.join(sub_dir_name);

    let mut config = Config {
        root_dir: internal_checks_sub_dir,

        //dependencies_crate_manifest_path: Some("Cargo.toml".into()),

        // OK to use `cargo clippy` to check both standard & clippy lints.

        //trailing_args: vec!["--".into(), "--test-threads".into(), "1".into()],
        program: CommandBuilder::cmd("cargo"),

        output_conflict_handling: OutputConflictHandling::Error,
        //mode: Mode::Panic,
        mode: Mode::Fail {
            require_patterns: false,
        },

        edition: None,
        ..Config::default()
    };

    config.program.args = vec![
        // @TODO test for clippy: and rustdoc:: lints.
        "test".into(),
        "--color".into(),
        "never".into(),
        "--quiet".into(),
        "--jobs".into(),
        "1".into(),
        "--no-fail-fast".into(),
        "--target-dir".into(),
        internal_checks_dir.parent().unwrap().join("target").into(),
        "--manifest-path".into(),
    ];

    // @TODO:
    // We do not filter anything Windows-specific (like "exit code" or backslashes), as we
    // don't support testing on Windows (because of symlinks).

    // avoid rendering github actions messages in the dog food tests as they'd
    // show up in the diff and thus fail CI.
    config.program.envs.push(("GITHUB_ACTION".into(), None));

    config
        .program
        .envs
        .push(("BLESS".into(), Some("false".to_string().into())));

    config.stdout_filter("in ([0-9]m )?[0-9\\.]+s", "");
    config.stderr_filter(r#""--out-dir"(,)? "[^"]+""#, r#""--out-dir"$1 "$$TMP"#);
    config.stderr_filter(
        "( *process didn't exit successfully: `[^-]+)-[0-9a-f]+",
        "$1-HASH",
    );
    // Windows io::Error uses "exit code".
    config.stderr_filter("exit code", "exit status");
    // The order of the `/deps` directory flag is flaky
    config.stderr_filter("/deps", "");

    // What is this "$DIR"?
    // This failed!

    //config.path_stderr_filter(&std::path::Path::new(internal_checks_dir), "$DIR");

    config.stderr_filter("[0-9a-f]+\\.rmeta", "$$HASH.rmeta");
    // Windows backslashes are sometimes escaped.
    // Insert the replacement filter at the start to make sure the filter for single backslashes
    // runs afterwards.
    config
        .stderr_filters
        .insert(0, (Match::Exact(b"\\\\".to_vec()), b"\\"));
    config.stderr_filter("\\.exe", b"");
    config.stderr_filter(r#"(panic.*)\.rs:[0-9]+:[0-9]+"#, "$1.rs");
    config.stderr_filter("   [0-9]: .*", "");
    config.stderr_filter("/target/[^/]+/debug", "/target/$$TRIPLE/debug");
    config.stderr_filter("(command: )\"[^<rp][^\"]+", "$1\"$$CMD");

    ui_test::run_tests_generic(config, |path| path.ends_with("Cargo.toml"), |_, _| None)
}
