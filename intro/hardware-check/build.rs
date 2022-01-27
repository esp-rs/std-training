use std::{
    fs::File,
    io::{Read, Write},
};

use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    let mut cfg_file = File::open("cfg.toml").expect("cfg.toml missing");
    let mut buf = vec![];
    cfg_file.read_to_end(&mut buf)?;
    drop(cfg_file);
    let mut cfg: toml::Value = toml::from_slice(&buf)?;

    const UUID_KEY: &'static str = "uuid";
    match &mut cfg {
        toml::Value::Table(kv) => match kv.get_mut(env!("CARGO_PKG_NAME")) {
            Some(toml::Value::Table(inner)) => {
                if !inner.contains_key(UUID_KEY) {
                    let uuid_val = Uuid::new_v4().to_string();
                    inner.insert(UUID_KEY.to_string(), toml::Value::String(uuid_val));

                    let cfg_out = toml::to_vec(&cfg)?;
                    let mut cfg_file = File::create("cfg.toml")?;
                    cfg_file.write_all(&cfg_out)?;
                }
            }
            _ => panic!("malformed cfg.toml"),
        },
        _ => panic!("malformed cfg.toml"),
    }

    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641

    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
}
