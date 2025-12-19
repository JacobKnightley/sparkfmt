use sparkfmt_core::format_sql;

#[test]
fn test_is_distinct_from() {
    let input = "select x from t where x is distinct from y";
    let expected = "SELECT\n     x\nFROM t\nWHERE x IS DISTINCT FROM y";
    assert_eq!(format_sql(input).unwrap(), expected);
}

#[test]
fn test_is_not_distinct_from() {
    let input = "select x from t where x is not distinct from y";
    let expected = "SELECT\n     x\nFROM t\nWHERE x IS NOT DISTINCT FROM y";
    assert_eq!(format_sql(input).unwrap(), expected);
}
