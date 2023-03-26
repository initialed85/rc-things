extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

use bevy_video::prelude::*;
use openh264::decoder::Decoder;
use openh264::nal_units;

#[macro_use]
#[cfg(test)]
mod tests {
    use openh264::decoder::DecoderConfig;

    use super::*;

    #[test]
    fn test_add() {
        let raw_data = std::fs::read("./parsed_dump.txt").unwrap();
        let packets = rmps::decode::from_slice::<Vec<Vec<u8>>>(&raw_data).unwrap();
        println!("packets={:?}", packets.len());

        let mut joined_packets = vec![];
        for packet in packets.clone() {
            joined_packets.extend_from_slice(&packet)
        }

        let mut decoder = Decoder::new().unwrap();
        for nal in nal_units(&joined_packets) {
            println!("nal={:?}", nal);

            let decoded_data = decoder.decode(nal);
            if decoded_data.is_err() {
                println!("decoded_data is err={:?}", decoded_data.err().unwrap());
                continue;
            }

            let decoded_data = decoded_data.unwrap();
            if decoded_data.is_none() {
                println!("decoded_data is none");
                continue;
            }

            let decoded_data = decoded_data.unwrap();
            println!("{:?}", decoded_data);

            let mut frame_data = vec![];
            decoded_data.write_rgb8(&mut frame_data);
        }

        assert_eq!(1, 2);
    }
}
