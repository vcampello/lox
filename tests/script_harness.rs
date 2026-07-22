mod common;

use common::Trap;
use datatest_stable::{self, Utf8Path};

const ROOT: &str = "tests/fixtures";

datatest_stable::harness! {
    // all files
    // { test = validate_success, root = ROOT, pattern = r".*\.lox$" },

    // all directories
    // { test = validate_success, root = ROOT, pattern = r"assignment/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"benchmark/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"block/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"bool/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"call/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"class/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"closure/.*\.lox$" },
    { test = validate_success, root = ROOT, pattern = r"comments/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"constructor/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"expressions/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"field/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"for/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"function/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"if/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"inheritance/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"limit/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"logical_operator/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"method/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"nil/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"number/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"operator/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"print/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"regression/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"return/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"scanning/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"string/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"super/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"test/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"this/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"variable/.*\.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"while/.*\.lox$" },

    // individual files
    { test = validate_success, root = ROOT, pattern = r"^empty_file.lox$" },
    { test = validate_success, root = ROOT, pattern = r"^precedence.lox$" },
    // { test = validate_success, root = ROOT, pattern = r"^unexpected_character.lox$" },
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
        if actual == expected {
            continue;
        }

        return Err(format!(
            "assertion {}/{}. {}:{}",
            out_line_num,
            assertions.len(),
            file_slug,
            line_num
        )
        .into());
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
