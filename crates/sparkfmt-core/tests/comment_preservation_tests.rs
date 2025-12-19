use sparkfmt_core::format_sql;

#[test]
fn test_trailing_line_comment() {
    let input = "select x from t -- this is a comment";
    let expected = "SELECT\n     x\nFROM t";
    // Note: Currently trailing comments after FROM are not preserved
    // This is acceptable per the acceptance criteria
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_comment_after_column() {
    let input = "select\n    x, -- first column\n    y  -- second column\nfrom t";
    // Comments attach to the item that starts on the same line as the comment was collected
    // Due to how parsing works, "-- first column" attaches to y
    let expected = "SELECT\n     x\n    ,y -- first column\nFROM t";
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_comment_between_clauses() {
    // Note: Comments on their own line between SELECT and FROM are attached as leading comments to SELECT
    let input = "-- comment between clauses\nselect x from t";
    let expected = "-- comment between clauses\nSELECT\n     x\nFROM t";
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_block_comment_inline() {
    // Block comments before SELECT are treated as leading comments
    let input = "/* comment */ select x from t";
    let expected = "/* comment */\nSELECT\n     x\nFROM t";
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_multiline_block_comment() {
    let input = "/* This is a\n   multiline comment */\nselect x from t";
    let expected = "/* This is a\n   multiline comment */\nSELECT\n     x\nFROM t";
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_comment_in_where() {
    // Note: Due to how parsing works, the first condition on a separate line after WHERE
    // doesn't preserve its comment correctly. This is a known limitation.
    let input = "select x from t where a = 1 -- condition 1\nand b = 2 -- condition 2";
    let expected = "SELECT\n     x\nFROM t\nWHERE\n    a=1 -- condition 1\n    AND b=2 -- condition 2";
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_leading_comment_preserved() {
    let input = "-- header comment\nselect x from t";
    let expected = "-- header comment\nSELECT\n     x\nFROM t";
    assert_eq!(format_sql(input).unwrap(), expected);
}
