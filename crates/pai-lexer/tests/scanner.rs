use insta::assert_snapshot;
use pai_file::SourceFile;
use pai_lexer::Lexer;

#[test]
fn main() {
    let source_file = SourceFile::read("tests/fixtures/demo.ts").unwrap();

    let lexer = Lexer::new(source_file.source());

    let content: String = lexer.map(|unit| format!("{unit:?}\n")).collect();

    assert_snapshot!(content);
}
