use crate::config::ConfigEntry;

pub fn config_lookup<'a>(entries: &'a [ConfigEntry], key: &str) -> Option<&'a str> {
    entries.iter().find(|e| e.key == key).map(|e| e.value.as_str())
}
