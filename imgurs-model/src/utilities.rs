/// Format json by parsing and pretty-printing it
pub fn pretty_json(text: &str) -> Result<String, serde_json::Error> {
    let v = serde_json::from_str::<serde_json::Value>(text)?;
    serde_json::to_string_pretty(&v)
}
