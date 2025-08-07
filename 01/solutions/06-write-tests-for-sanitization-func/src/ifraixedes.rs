/// This function implements a string sanitization logic that should uphold the following
/// properties:
/// - After sanitization, the result must not end with the character `x`
/// - After sanitization, the result must not end with the character `o`
/// - After sanitization, the result must not end with the string `.exe`
///
/// The function assumes that the input to the function only consists of lower and uppercase
/// characters from the English alphabet and digits 0-9.
///
/// The implementation contains some bugs.
///
/// Your task is to write a set (at least 8) of unit tests, use them to find (at least 2) bugs in
/// this function and then fix the function.
fn sanitize(input: &str) -> &str {
    let mut input = input;

    loop {
        let prev_len = input.len();

        // Remove all x from the end of the string
        input = input.trim_end_matches("x");
        // Remove all o from the end of the string
        input = input.trim_end_matches("o");
        // Remove .exe from the end
        input = input.trim_end_matches(".exe");

        if prev_len == input.len() {
            break;
        }
    }

    input
}

/// TODO: write tests for the `sanitize` function
///
/// Bonus: can you find any bugs using the [proptest](https://proptest-rs.github.io/proptest/intro.html)
/// crate?
/// Note that you will probably need to run `cargo test` with the `PROPTEST_DISABLE_FAILURE_PERSISTENCE=1`
/// environment variable to make proptest work for tests stored in the `tests` directory.
#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn clean_input() {
        assert_eq!(sanitize(""), "", "empty");
        assert_eq!(sanitize("b"), "b", "one lower letter");
        assert_eq!(sanitize("if"), "if", "two lower letters");
        assert_eq!(sanitize("bar"), "bar", "three lowers letters");
        assert_eq!(sanitize("W"), "W", "one upper letter");
        assert_eq!(sanitize("YC"), "YC", "two upper letters");
        assert_eq!(sanitize("BOT"), "BOT", "three upper letters");
        assert_eq!(
            sanitize("FooBar"),
            "FooBar",
            "upper and lower letters starting with upper"
        );
        assert_eq!(
            sanitize("lowHigh"),
            "lowHigh",
            "upper and lower letters starting with lower"
        );
        assert_eq!(sanitize("O"), "O", "just 'O'");
        assert_eq!(sanitize("X"), "X", "just 'X'");
        assert_eq!(sanitize(".EXE"), ".EXE", "just '.EXE'");
        assert_eq!(sanitize("wow.o."), "wow.o.", "just '.EXE'");
        assert_eq!(sanitize("wow.x."), "wow.x.", "just '.EXE'");
        assert_eq!(sanitize("wow.x."), "wow.x.", "just '.EXE'");
        assert_eq!(sanitize("wow.exe."), "wow.exe.", "just '.EXE'");
        assert_eq!(sanitize("wow.exe."), "wow.exe.", "just '.EXE'");
        assert_eq!(sanitize("wow.exeox."), "wow.exeox.", "just '.EXE'");
    }

    #[test]
    fn inputs_ending_with_o() {
        assert_eq!(sanitize("o"), "", "just 'o'");
        assert_eq!(sanitize("bo"), "b", "two lower letters, last is 'o'");
        assert_eq!(sanitize("Ro"), "R", "one upper letter plus 'o'");
        assert_eq!(
            sanitize("readOnlyo"),
            "readOnly",
            "mix upper lower letters ending with 'o'"
        );
        assert_eq!(sanitize("ooo"), "", "just 'ooo'");
        assert_eq!(
            sanitize("ALotOfooooo"),
            "ALotOf",
            "mix upper lower letters ending with 'ooooo'"
        );
    }

    #[test]
    fn inputs_ending_with_x() {
        assert_eq!(sanitize("x"), "", "just x");
        assert_eq!(sanitize("yx"), "y", "two lower letters, last is 'x'");
        assert_eq!(sanitize("Tx"), "T", "one upper letter plus 'x'");
        assert_eq!(
            sanitize("teoDorusx"),
            "teoDorus",
            "mix upper and lower letters ending with 'x'"
        );
        assert_eq!(sanitize("xxxx"), "", "just 'xxxx'");
        assert_eq!(
            sanitize("WowHaxx"),
            "WowHa",
            "mix upper and lower letters ending with 'xx'"
        );
    }

    #[test]
    fn inputs_ending_with_dot_exe() {
        assert_eq!(sanitize(".exe"), "", "just '.exe'");
        assert_eq!(
            sanitize("myfile.exe"),
            "myfile",
            "word with lower letters ending with '.exe'"
        );
        assert_eq!(
            sanitize("MYFILE.exe"),
            "MYFILE",
            "word with up letters ending with '.exe'"
        );
        assert_eq!(
            sanitize("myFile.exe"),
            "myFile",
            "mix upper and lower letters ending with '.exe'"
        );
        assert_eq!(sanitize(".exe.exe.exe"), "", "just '.exe.exe.exe'");
        assert_eq!(
            sanitize("win.exe.exe"),
            "win",
            "mix upper and lower letters ending with '.exe.exe'"
        );
    }

    #[test]
    fn inputs_ending_with_combined_o_x_dot_exe() {
        assert_eq!(sanitize("o.exe"), "", "just 'o.exe'");
        assert_eq!(sanitize("x.exe"), "", "just 'x.exe'");
        assert_eq!(sanitize("ox.exe"), "", "just 'ox.exe'");
        assert_eq!(sanitize("xo.exe"), "", "just 'xo.exe'");
        assert_eq!(sanitize(".exeo"), "", "just '.exeo'");
        assert_eq!(sanitize(".exex"), "", "just '.exex'");
        assert_eq!(sanitize(".exeox"), "", "just '.exeox'");
        assert_eq!(sanitize(".exexo"), "", "just '.exexo'");
        assert_eq!(sanitize("yup.exeo"), "yup", "ending with '.exeo'");
        assert_eq!(sanitize("bee.exex"), "bee", "ending with '.exex'");
        assert_eq!(sanitize("yup.exeoo"), "yup", "ending with '.exeoo'");
        assert_eq!(sanitize("bee.exexx"), "bee", "ending with '.exexx'");
        assert_eq!(sanitize("yup.exeox"), "yup", "ending with '.exeox'");
        assert_eq!(sanitize("bee.exexo"), "bee", "ending with '.exexo'");
        assert_eq!(sanitize("myfileo.exe"), "myfile", "ending with 'o.exe'");
        assert_eq!(sanitize("wowx.exe"), "wow", "ending with 'x.exe'");
        assert_eq!(sanitize("billox.exe"), "bill", "ending with 'ox.exe'");
        assert_eq!(sanitize("billxo.exe"), "bill", "ending with 'xo.exe'");
        assert_eq!(
            sanitize("git.exe.exeox.exex.exeo"),
            "git",
            "ending with '.exe.exeox.exex.exeo'"
        );
        assert_eq!(
            sanitize("svnxo.exe.exeoxxo.exexxoo.exeoxoooxoxo"),
            "svn",
            "ending with '.exe.exeoxxo.exexxoo.exeoxoooxoxo'"
        );
    }
}

#[cfg(test)]
mod porptest {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mixed_inputs(first_part in ".*".prop_filter("input cannot ends with 'o', 'x', or '.exe'",
                |first_part| !(first_part.ends_with(['o', 'x']) || first_part.ends_with(".exe"))),
                last_part in r#"(o|x|\.exe)*"#,
        ) {
           let  input = format!("{first_part}{last_part}");
           let sanitized = sanitize(&input);
            prop_assert_eq!(first_part, sanitized);
        }

    }
}
