use nu_test_support::{nu, pipeline};

#[test]
fn can_average_numbers() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
             open sgml_description.json
             | get glossary.GlossDiv.GlossList.GlossEntry.Sections
             | math average
             | echo $it
         "#
    ));
    println!("{:?}", actual.err);
    assert_eq!(actual.out, "101.5")
}

#[test]
fn can_average_bytes() {
    let actual = nu!(
        cwd: "tests/fixtures/formats",
        "ls | sort-by name | skip 1 | first 2 | get size | math average | format \"{$it}\" | echo $it"
    );

    assert_eq!(actual.out, "1.6 KB");
}
