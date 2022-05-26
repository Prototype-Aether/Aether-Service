use std::{
    array::TryFromSliceError,
    convert::{TryFrom, TryInto},
    io::{stdout, Write},
};

use aether_lib::util::{compile_u16, compile_u32};

#[derive(Clone, PartialEq, Debug)]
struct Arg {
    pub length: u16,
    pub arg: Vec<u8>,
}

impl TryFrom<&[u8]> for Arg {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        stdout().flush().unwrap();
        let length = u16::from_be_bytes(value[0..2].try_into()?);
        let arg: Vec<u8> = value[2..(2 + length as usize)].into();
        Ok(Arg { length, arg })
    }
}

impl From<Arg> for Vec<u8> {
    fn from(value: Arg) -> Self {
        let mut result: Self = Vec::new();

        result.extend(compile_u16(value.length));
        result.extend(value.arg);

        result
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Message {
    pub application_id: u32,
    pub message_type: u16,
    pub argc: u16,
    pub args: Vec<Arg>,
}

impl TryFrom<&[u8]> for Message {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let application_id = u32::from_be_bytes(value[0..4].try_into()?);
        let message_type = u16::from_be_bytes(value[4..6].try_into()?);
        let argc = u16::from_be_bytes(value[6..8].try_into()?);

        let mut args: Vec<Arg> = Vec::new();
        // start reading from the next index
        let mut index: usize = 8;

        for _ in 0..argc {
            let arg = Arg::try_from(&value[index..]).unwrap();
            index += 2 + arg.length as usize;
            args.push(arg);
        }

        Ok(Message {
            application_id,
            message_type,
            argc,
            args,
        })
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        let mut result: Self = Vec::new();

        result.extend(compile_u32(value.application_id));
        result.extend(compile_u16(value.message_type));
        result.extend(compile_u16(value.argc));

        // convert each arg to Vec<u8> using map
        let args: Vec<Vec<u8>> = value.args.into_iter().map(From::from).collect();

        result.extend(args.concat());

        result
    }
}

pub fn handle_command(buf: &[u8]) {
    let command = Message::try_from(buf);
}

#[cfg(test)]
mod tests {
    use std::{
        convert::TryFrom,
        io::{stdout, Write},
    };

    use aether_lib::util::gen_nonce;
    use rand::{random, thread_rng, Rng};

    use super::{Arg, Message};

    #[test]
    fn convert_test() {
        // test on these number of random test cases
        let num_tests = 100;
        for _ in 0..num_tests {
            let application_id: u32 = random();
            let message_type: u16 = random();
            let argc: u16 = thread_rng().gen_range(0..32);

            let args: Vec<Arg> = (0..argc)
                .map(|_| {
                    let length: u16 = random();
                    let arg = gen_nonce(length as usize);
                    Arg { length, arg }
                })
                .collect();

            let message = Message {
                application_id,
                message_type,
                argc,
                args,
            };

            let message_bytes: Vec<u8> = message.clone().into();

            println!("Len: {}", message_bytes.len());
            stdout().flush().unwrap();

            let message_out: Message = Message::try_from(message_bytes.as_slice()).unwrap();

            assert_eq!(message, message_out);
        }
    }
}
