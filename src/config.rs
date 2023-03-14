use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub zmq_port: usize,
    pub zmq_no_wait: bool,
}

impl Config {
    pub fn read(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read_to_string(path)?;
        let c: Config = serde_json::from_str(data.as_str())?;

        Ok(c)
    }
}
