pub const UID_INCREMENT_QUERY: &str = "RETURN fn::increment($counter_name)";
pub const UID_COUNTER_NAME_BIND: &str = "counter_name";

pub const DEFINE_INCREMENT_FUNC_QUERY: &str = r#"DEFINE FUNCTION fn::increment($name: string) {
    RETURN (UPSERT ONLY type::thing('counter', $name)
    SET value += 1).value;
};"#;
