pub fn key_to_string(key: rdev::Key) -> String {
    serde_variant::to_variant_name(&key)
        .unwrap_or("unknown")
        .to_string()
}

pub fn string_to_key(key: String) -> Option<rdev::Key> {
    let cheating = format!("\"{key}\"");
    serde_json::from_str::<rdev::Key>(&cheating).ok()
}
