mod parse;

pub fn run() {
    // json
    println!(
        "{:#?}",
        parse::parse_source(nano_parser_gen::lexer::SourceFile::from(
            r#"{
              "string": "test",
              "number": 123,
              "null": null,
              "array": [ 1, 2, 3 ],
              "booleans": {
                "true": true,
                "false": false
              }
            }"#,
        ))
    );
}
