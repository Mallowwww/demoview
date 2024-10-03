use std::io::{Error, ErrorKind, Read};

/// Turns a bytes vector into an integer.
/// First argument should be a bytes vector, second argument should be the integer type you want to decode. 
#[macro_export]
macro_rules! bytes2int {
    ($bytes_to_read:expr, $int_type:ident) => {
        $int_type::from_le_bytes($bytes_to_read)
            .try_into()
            .expect("bytes2int was called with invalid parameters.")
    }
}


/// Helping tool to automatically read an integer a file, or anything that fits the criteria for the first argument.
/// First argument is expected to be anything that implements the Read trait, but it simply calls the read_exact method,
/// second argument should be an integer type. 
#[macro_export]
macro_rules! buf_read_int {
    ($file_to_read:ident, $int_type:ident) => {
        {
            let mut _macro_temp: [u8; size_of::<$int_type>()] = [0; size_of::<$int_type>()];
            $file_to_read.read_exact(_macro_temp.as_mut_slice()).expect("file_read_int failed to read from file. Bad buffer?");
            bytes2int!(_macro_temp, $int_type)
        }
    }
}

pub(crate) use {bytes2int, buf_read_int};

/// Reads from file until it reaches a string terminator byte (`\x00`), then returns it as `String`.
/// `greedy` determines whether it will read `max_size` from the buffer then determine the string to extract, if it is,
/// or if it should read one byte at a time until it reaches a string terminator byte.
pub fn read_string_from_buf(buf_to_read: &mut impl Read, max_size: usize, greedy: bool) -> Result<String, Error> {
    let mut string_buf: Vec<u8> = vec![0; max_size];

    match greedy {
        true => {
            buf_to_read.read_exact(string_buf.as_mut_slice())?;

            for cur_byte in 0..max_size {
                
                if string_buf[cur_byte] == 0x00 {
                    string_buf.truncate(cur_byte);
                    break
                }

            }
        }
        false => {
            for _ in 0..max_size {
                let read_byte: &mut [u8] = &mut [0];
                buf_to_read.read_exact(read_byte)?;
                
                if read_byte[0] == 0x00 {
                    break
                }
                
                string_buf.push(read_byte[0]);
            }
        
        }
    }

    let ret_string = String::from_utf8(string_buf)
        .or(Err(Error::new(ErrorKind::InvalidData, "INVALID_STRING")))?;

    Ok(ret_string)
}