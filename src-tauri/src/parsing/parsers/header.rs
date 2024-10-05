// size of 1072
use std::io::{Error, ErrorKind, Read, BufReader};
use std::fs::File;
use std::collections::VecDeque;

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::parsing::utils::*;
use crate::parsing::parsers::common::Parser;

/// The universal header for Source's `.dem` files.
/// Note that the file stamp is not included, because any valid demo header always has "HL2DEMO\x00" as a file stamp.
#[derive(Debug)]

#[allow(dead_code)]
pub struct DemoHeader {
    demo_protocol : u32,
    net_protocol  : u32,
    server_name   : String,
    client_name   : String,
    map_name      : String,
    game_dir      : String,
    playback_time : f32,
    tick_count    : i32,
    frame_count   : i32,
    signon_length : u32,
}

impl Parser for DemoHeader {

    /// Reads and parses demo data from file and then returns a new object with all it's information.
    fn parse_impl(data_to_read: &mut VecDeque<u8>) -> Result<Self, Error> {
        let dem_signature = read_string_from_buf(data_to_read, 8, true)?;

        if dem_signature != "HL2DEMO" {
            println!("{dem_signature:?}");
            return Err( Error::new(ErrorKind::InvalidData, "NOT_A_DEMO"));
        }
        
        // Great! If it's here, it means it's good to go.
        let demo_protocol: u32 = buf_read_int!(data_to_read, u32);
        let net_protocol: u32 = buf_read_int!(data_to_read, u32);
        let server_name: String = read_string_from_buf(data_to_read, 260, true)?; 
        let client_name: String = read_string_from_buf(data_to_read, 260, true)?;
        let map_name: String = read_string_from_buf(data_to_read, 260, true)?;
        let game_dir: String = read_string_from_buf(data_to_read, 260, true)?;
        let playback_time: f32 = buf_read_int!(data_to_read, f32);
        let tick_count: i32 = buf_read_int!(data_to_read, i32);
        let frame_count: i32 = buf_read_int!(data_to_read, i32);
        let signon_length: u32 = buf_read_int!(data_to_read, u32);

        Ok(Self {
            demo_protocol,
            net_protocol,
            server_name,
            client_name,
            map_name,
            game_dir,
            playback_time,
            tick_count,
            frame_count,
            signon_length,
        })
    }

    fn read_size(_: &mut BufReader<File>) -> Result<usize, Error> {
        Ok(1072)
    }
}
impl Serialize for DemoHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DemoHeader", 10)?;
        s.serialize_field("demo_protocol", &self.demo_protocol)?;
        s.serialize_field("net_protocol", &self.net_protocol)?;
        s.serialize_field("server_name", &self.server_name)?;
        s.serialize_field("client_name", &self.client_name)?;
        s.serialize_field("game_dir", &self.game_dir)?;
        s.serialize_field("playback_time", &self.playback_time)?;
        s.serialize_field("tick_count", &self.tick_count)?;
        s.serialize_field("frame_count", &self.frame_count)?;
        s.serialize_field("signon_length", &self.signon_length)?;
        s.end()
    }
}