use rgb::ComponentSlice;
use std::{
    borrow::{Borrow, Cow},
    str,
};

pub use rgb::RGB8;

/// Handles `EspMqttMessage` with MQTT hierarchy
///
/// Can be used to send ColorData(rgb) with `Command` in a hierarchical context
pub fn cmd_topic_fragment(uuid: &str) -> String {
    format!("{}/command/", uuid)
}

/// Handles `EspMqttMessage` without MQTT hierarchy
///
/// Used to send ColorData(rgb)
pub fn color_topic(uuid: &str) -> String {
    format!("{}/color_topic", uuid)
}

pub fn temperature_data_topic(uuid: &str) -> String {
    format!("{}/sensor_data/temperature", uuid)
}

pub fn hello_topic(uuid: &str) -> String {
    format!("{}/hello", uuid)
}

pub enum Command {
    BoardLed(RGB8),
}

impl Command {
    const BOARD_LED: &'static str = "board_led";

    pub fn topic(&self, uuid: &str) -> String {
        match self {
            Command::BoardLed(_) => {
                format!("{}{}", cmd_topic_fragment(uuid), Self::BOARD_LED)
            }
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            Command::BoardLed(led_data) => led_data.as_slice(),
        }
    }
}

/// `ColorData` is a simplified `Command`
pub enum ColorData {
    BoardLed(RGB8),
}
impl ColorData {
    pub fn topic(&self, uuid: &str) -> String {
        match self {
            ColorData::BoardLed(_) => color_topic(uuid),
        }
    }
    pub fn data(&self) -> &[u8] {
        match self {
            ColorData::BoardLed(led_data) => led_data.as_slice(),
        }
    }
}
#[derive(Debug)]
pub struct RawCommandData<'a> {
    pub path: &'a str,
    pub data: Cow<'a, [u8]>,
}

impl<'a> TryFrom<Command> for RawCommandData<'a> {
    type Error = ();

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        match value {
            Command::BoardLed(rgb) => Ok(RawCommandData {
                data: Cow::Owned(vec![rgb.r, rgb.g, rgb.b]),
                path: Command::BOARD_LED,
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
        //if value.path == Command::BOARD_LED {
        if value.path == "" {
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

/// Handles `.data()` from EspMqttMessage
///
// The message is a slice containing 3 values, and is cast into a ColorData(rgb)
impl<'a> TryFrom<&[u8]> for ColorData {
    type Error = ConvertError;

    fn try_from(message: &[u8]) -> Result<Self, Self::Error> {
        if message.len() == 3 {
            let rgb = RGB8::new(message[0], message[1], message[2]);
            Ok(ColorData::BoardLed(rgb))
        } else {
            Err(ConvertError::Length(message.len()))
        }
    }
}
