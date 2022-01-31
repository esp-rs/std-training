include!(concat!(env!("CARGO_MANIFEST_DIR"), "/_uuid.rs"));

pub const fn uuid() -> &'static str {
    UUID
}
