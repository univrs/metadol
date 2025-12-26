//! Golden file tests for code generation
//! Compare generated output against expected files

use metadol::codegen::RustCodegen;
use metadol::parser::Parser;

fn normalize(s: &str) -> String {
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

macro_rules! golden_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!("codegen/golden/input/", stringify!($name), ".dol"));
            let expected = include_str!(concat!(
                "codegen/golden/expected/",
                stringify!($name),
                ".rs"
            ));

            let ast = Parser::new(input).parse_file().expect("Parse failed");
            // Use static method form since generate takes &Declaration
            let actual = ast
                .declarations
                .iter()
                .map(|decl| RustCodegen::generate(decl))
                .collect::<Vec<_>>()
                .join("\n\n");

            assert_eq!(
                normalize(&actual),
                normalize(expected),
                "\n=== ACTUAL ===\n{}\n=== EXPECTED ===\n{}",
                actual,
                expected
            );
        }
    };
}

golden_test!(simple_gene);
golden_test!(gene_with_fields);
golden_test!(function);
// golden_test!(pipe_operators);
