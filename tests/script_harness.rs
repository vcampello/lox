mod common;
use common::Trap;
use datatest_stable::{self, Utf8Path};

datatest_stable::harness! {
    {
        test = validate_success,
        root = "samples",
        pattern = r".*\.lox$"
    },
    // {
    //     test = validate_success,
    //     root = "samples",
    //     pattern = r"^logical_operator/.*\.lox$"
    // },
}

fn validate_success(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
    let file_slug = format!("file: {}", path);
    let assertions = extract_assertions(&contents);

    let (trap, mut rt) = Trap::new_runtime();
    rt.run(&contents).map_err(|e| format!("{e}"))?;

    // just run the script
    if assertions.is_empty() {
        return Ok(());
    }

    let actual = trap.to_lines();

    assert_eq!(
        actual.len(),
        assertions.len(),
        "captured {} output lines but expected {}. {}",
        actual.len(),
        assertions.len(),
        file_slug
    );

    // evaluate every output lines against the expected value
    for (out_line_num, (actual, (line_num, expected))) in
        actual.iter().zip(assertions.iter()).enumerate()
    {
        assert_eq!(
            actual,
            expected,
            "assertion {}/{}. {}:{}",
            out_line_num,
            assertions.len(),
            file_slug,
            line_num
        );
    }

    Ok(())
}

/// Find `// expect:` statements in the script and extract the expected value.
/// Returns the line number and stdout value of the expect statement
fn extract_assertions(script: &str) -> Vec<(usize, &str)> {
    let prefix = "// expect:";
    script
        .lines()
        .enumerate()
        .filter_map(|(line_num, line)| {
            line.find(prefix)
                .map(|index| (line_num, line[index + prefix.len()..].trim()))
        })
        .collect()
}
