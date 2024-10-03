use std::io::{BufReader, Error, Read};
use std::fs::File;
use std::collections::VecDeque;

pub trait Parser {

    /// DO NOT IMPLEMENT THIS! LEAVE IT AS IS!
    /// Instead, implement `parse_impl` and `read_size`, to actually parse the file and to know how much you need to read at once, respectivelly.
    /// This is what you're supposed to call to parse the file.
    fn parse(data_to_read: &mut BufReader<File>) -> Result<Self, Error>
    where 
        Self: Sized 
    {
        let expected_read_size = Self::read_size(data_to_read)?;
        let mut parse_buffer: Vec<u8> = vec![0; expected_read_size];

        data_to_read.read_exact(parse_buffer.as_mut_slice())?;

        Self::parse_impl(&mut parse_buffer.into())
    }

    /// This is what actually unpacks the read data from the file into the struct that is implmeneting this trait.
    fn parse_impl(data_to_read: &mut VecDeque<u8>) -> Result<Self, Error>
    where 
        Self: Sized;
    
    /// This should return how many bytes the code should read from the 
    fn read_size(data_to_read: &mut BufReader<File>) -> Result<usize, Error>;
}