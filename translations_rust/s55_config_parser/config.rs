pub const MAX_ENTRIES: usize = 64;

#[derive(Clone)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
}
