use std::borrow::{Borrow, Cow};

use rgb::ComponentSlice;
pub use rgb::RGB8;

// not really a topic, since it ends with a trailing slash
pub fn cmd_topic_fragment(uuid: &str) -> String {
    format!("{}/cmd/", uuid)
}

pub fn temperature_data_topic(uuid: &str) -> String {
    format!("{}/sensor_data/mcu_temp", uuid)
}

pub enum Command {
    BoardLed(RGB8),
}

impl Command {
    const BOARD_LED: &'static str = "led";

    pub fn topic(&self, uuid: &str) -> String {
        match self {
            Command::BoardLed(_) => format!("{}{}", cmd_topic_fragment(uuid), Self::BOARD_LED),
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            Command::BoardLed(led_data) => led_data.as_slice(),
        }
    }
}

pub struct RawCommandData<'a> {
    pub path: Cow<'a, str>,
    pub data: Cow<'a, [u8]>,
}

impl<'a> TryFrom<Command> for RawCommandData<'a> {
    type Error = ();

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        match value {
            Command::BoardLed(rgb) => Ok(RawCommandData {
                data: Cow::Owned(vec![rgb.r, rgb.g, rgb.b]),
                path: Cow::Borrowed(Command::BOARD_LED),
            }),
        }
    }
}

pub enum ConvertError {
    Length(usize),
    InvalidPath,
}

impl<'a> TryFrom<RawCommandData<'a>> for Command {
    type Error = ConvertError;

    fn try_from(value: RawCommandData) -> Result<Self, Self::Error> {
        if value.path == Command::BOARD_LED {
            let data: &[u8] = value.data.borrow();
            let data: [u8; 3] = data
                .try_into()
                .map_err(|_| ConvertError::Length(data.len()))?;
            let rgb = RGB8::new(data[0], data[1], data[2]);
            Ok(Command::BoardLed(rgb))
        } else {
            Err(ConvertError::InvalidPath)
        }
    }
}
