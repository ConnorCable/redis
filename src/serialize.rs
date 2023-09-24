#[allow(dead_code)]
pub mod resp {
    use regex::Regex;
    use std::collections::VecDeque;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum RespIdentifier {
        RespSimpleStr,
        RespSimpleErr,
        RespInt,
        RespBulkStr,
        RespArray,
        RespNull,
        RespBool,
        RespBigNumber,
        RespVerbString,
        RespMap,
        RespSet,
        RespPush,
    }

    pub struct SerializedContainer {
        pub length: u32,
        pub messages: Vec<SerializedMessage>,
    }

    pub struct SerializedMessage {
        pub length: usize,
        pub identifier: RespIdentifier,
        pub message: String,
    }

    trait UpdateLength {
        fn update(&mut self, entry: SerializedMessage);
        fn update_with_array(&mut self, entries: Vec<SerializedMessage>);
    }

    impl UpdateLength for SerializedContainer {
        fn update(&mut self, entry: SerializedMessage) {
            self.messages.push(entry);
            self.length += 1;
        }
        fn update_with_array(&mut self, entries: Vec<SerializedMessage>) {
            for entry in entries {
                self.length += 1;
                self.messages.push(entry)
            }
        }
    }

    fn ret_err_message(error: &str) -> SerializedMessage {
        let error_message = SerializedMessage {
            length: error.len(),
            identifier: RespIdentifier::RespSimpleErr,
            message: error.to_string(),
        };

        return error_message;
    }
    // receives
    fn serialize_bulk_string(content: &str) -> SerializedMessage {
        return SerializedMessage {
            length: (content.len() - 2),
            identifier: RespIdentifier::RespBulkStr,
            message: content[2..].to_string(),
        };
    }

    // splits an entry into its identifier and possible information
    // if boolean is true -> the entry encapsulates everything about itself
    // if boolean is false -> the entry informs the identity and length of the next entry
    // eg: +OK returns (RespIdentifier:RespSimpleStr , OK, true)
    // eg: $5 returns (RespIdentifier::RespBulkStr, 5, false)
    fn entry_splitter(entry: &str) -> (RespIdentifier, String, bool) {
        let mut split: Vec<&str> = entry.split("").filter(|x| !x.is_empty()).collect();
        if split.len() < 2 {
            return (
                RespIdentifier::RespSimpleErr,
                String::from("No message / length to be parsed"),
                true,
            );
        }

        let identifier = detect_identifier(split.remove(0));

        let message = split.concat();
      //  println!("message: {}", message);

        match identifier {
            RespIdentifier::RespBulkStr => (RespIdentifier::RespBulkStr, message, false),
            _ => (identifier, message, true),
        }
    }
    // receives m
    pub fn serialize_array(message: &str) -> Vec<SerializedMessage> {
        // TODO: NULL ARRAY

        let mut split: VecDeque<&str> = message.split("\r\n").filter(|x| !x.is_empty()).collect();

        let mut serialized_array = Vec::new();
        // "*2\r\n$4\r\necho\r\n$5\r\nhello world\r\n”
        // $4 , echo , $5 , hello world
        // *3\r\n:1\r\n:2\r\n:3\r\n
        // :1 , :2 , :3
        while !split.is_empty() {
            /*
            if the chunk identifier is not a bulk string,
            split the identifier from the data, and return a SerializedMessage with the data attached

            if the chunk identifier is a bulk string,
                seperate the identifier from the informing length
                check that the informing length is == actual length of the string
                pop the next message as the data for this and to attach to the SerializedMessage
                if the informed length of the string is != actual length of the string
                return an error SerializedMessage instead
            */

            let (id, data, encapsulated) = entry_splitter(split.pop_front().unwrap());
            if encapsulated {
                serialized_array.push(SerializedMessage {
                    length: data.len(),
                    identifier: id,
                    message: data,
                });
                continue;
            }
            // if this is a bulk string
            let payload = match split.pop_front() {
                Some(n) => n,
                None => {
                    serialized_array.push(ret_err_message("No payload string found!"));
                    continue;
                }
            };
          //  println!("Payload: {}", payload);
            // parse the length from data
            let payload_length = match data.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    serialized_array.push(ret_err_message("payload length could not be parsed!"));
                    continue;
                }
            };

          //  println!("Payload length: {}", payload_length);
          //  println!("Data: {}", data);
          //  println!("Data length: {}", data.len());

            if payload_length !=  payload.len() {
                serialized_array.push(ret_err_message(
                    "Informing length does not equal payload length!",
                ));
                continue;
            }

            serialized_array.push(SerializedMessage {
                length: payload_length,
                identifier: id,
                message: payload.to_string(),
            });
        }

        return serialized_array;
    }
    // receive the message
    // if it is a simple message, process it with process_resp_simple
    // if it is an array, process it with serialize_array
    pub fn construct_serialized_message(message: &str) -> SerializedContainer {
        let mut container = SerializedContainer {
            length: 0,
            messages: Vec::new(),
        };
        let identifier = detect_identifier(&message[0..1]);
        match identifier {
            RespIdentifier::RespArray => {
                container.update_with_array(serialize_array(&message[2..]))
            }
            RespIdentifier::RespBulkStr => container.update(serialize_bulk_string(message)),
            RespIdentifier::RespInt | RespIdentifier::RespBigNumber => {
                container.update(serialize_simple(message, identifier))
            }
            RespIdentifier::RespBool
            | RespIdentifier::RespSimpleStr
            | RespIdentifier::RespSimpleErr => {
                container.update(serialize_simple(&message, identifier))
            }
            _ => container.update(ret_err_message("Identifier not parsed")),
        };

        return container;
    }

    fn detect_identifier(c: &str) -> RespIdentifier {
        match c {
            "+" => RespIdentifier::RespSimpleStr,
            "-" => RespIdentifier::RespSimpleErr,
            ":" => RespIdentifier::RespInt,
            "#" => RespIdentifier::RespBool,
            "*" => RespIdentifier::RespArray,
            "$" => RespIdentifier::RespBulkStr,
            _ => RespIdentifier::RespSimpleErr,
        }
    }
    // captures the human readable string of the split message
    pub fn serialize_simple(message: &str, id: RespIdentifier) -> SerializedMessage {
        let capture_group = Regex::new(r".(.+)\r\n").unwrap();
        if let Some(captures) = capture_group.captures(message) {
            let captured = captures.get(1).unwrap().as_str();
            return SerializedMessage {
                length: captured.len(),
                message: captured.to_string(),
                identifier: id,
            };
        } else {
            return ret_err_message("No message could be captured!");
        }
    }
}
