#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputMessage {
    // Ackermann steering
    pub throttle: f32,
    pub steering: f32,
    // Differential steering
    pub throttle_left: f32,
    pub throttle_right: f32,
    // Other
    pub mode_up: bool,
    pub mode_down: bool,
    pub mode_left: bool,
    pub mode_right: bool,
    pub handbrake: bool,
}

pub fn serialize<T>(t: T) -> Result<Vec<u8>, rmp_serde::encode::Error>
where
    T: serde::ser::Serialize,
{
    rmp_serde::to_vec(&t)
}

pub fn deserialize<T>(message: Vec<u8>) -> Result<T, rmp_serde::decode::Error>
where
    T: serde::de::DeserializeOwned,
{
    rmp_serde::from_slice::<T>(message.as_slice())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_and_deserialize() -> anyhow::Result<()> {
        let input_message = InputMessage {
            throttle: 0.69,
            steering: 0.69,
            throttle_left: 0.69,
            throttle_right: 0.69,
            mode_up: true,
            mode_down: true,
            mode_left: true,
            mode_right: true,
            handbrake: true,
        };

        let serialized_input_message = serialize(input_message.clone())?;

        let deserialized_input_message = deserialize(serialized_input_message)?;

        assert_eq!(input_message, deserialized_input_message);

        Ok(())
    }
}
