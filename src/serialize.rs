#[allow(dead_code)]
pub mod resp {
    use regex::Regex;
    use tokio::net::windows::named_pipe::ServerOptions;
    use std::collections::VecDeque;

    #[derive(Debug, PartialEq, Eq)]
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
        fn update(&self, entry: SerializedMessage);
        fn update_with_array(&self, entries: Vec<SerializedMessage>);
    }

    impl UpdateLength for SerializedContainer {
        fn update(&self, entry: SerializedMessage) {
            self.messages.push(entry);
            self.length += 1;
        }
        fn update_with_array(&self, entries: Vec<SerializedMessage>) {
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
            
        }

        let identifier = detect_identifier(split.remove(0));

        let message = split.concat();

        match identifier {
            RespIdentifier::RespInt => (identifier, message, true),
            RespIdentifier::RespBigNumber => (identifier, message, true),
            RespIdentifier::RespBool => (identifier, message, true),
            RespIdentifier::RespSimpleStr => (identifier, message, true),
            _ => (identifier, message, true),
        }




    }
    // receives m
    pub fn serialize_array(message: &str, length: &str) -> Vec<SerializedMessage> {
        let arr_length: usize;
        // TODO: NULL ARRAY

        match length.parse::<usize>() {
            Ok(n) => arr_length = n,
            Err(e) => return vec![ret_err_message("Array length not parsed correctly!")],
        }


        let mut split: VecDeque<&str> = message.split("\r\n").filter(|x| !x.is_empty()).collect();
        


        let mut serialized_array = Vec::new();
        // "*2\r\n$4\r\necho\r\n$5\r\nhello world\r\n”
        // $4 , echo , $5 , hello world
        // *3\r\n:1\r\n:2\r\n:3\r\n
        // :1 , :2 , :3
        while !split.is_empty() {
            let (id , length_or_entry, encapsulates) = entry_splitter(split.pop_front().unwrap());
            let serialized_message: SerializedMessage;
            if encapsulates {
                serialized_message = SerializedMessage {
                    identifier : id,
                    message : length_or_entry,
                    length : length_or_entry.len()
                };
                serialized_array.push(serialized_message);
                continue;
            }
            if let Ok(entry_length) = length_or_entry.parse::<usize>() {
                let entry_message = split.pop_front().unwrap();
                serialized_message = SerializedMessage {
                    identifier : id,
                    length : entry_length,
                    message: split.pop_front().unwrap().to_string()
                };
                serialized_array.push(serialized_message);
                continue;

            }
            else{
                // if the parse isn't sucessful, discard the next 
                split.pop_front();
                serialized_array.push(ret_err_message("Error parsing length"))
            }
            

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
                container.update_with_array(serialize_array(&message[2..], &message[1..1]))
            }
            RespIdentifier::RespBulkStr => container.update(serialize_bulk_string(message)),
            RespIdentifier::RespInt | RespIdentifier::RespBigNumber => {
                container.update(serialize_int(message))
            }
            RespIdentifier::RespBool
            | RespIdentifier::RespSimpleStr
            | RespIdentifier::RespSimpleErr => {
                container.update(serialize_simple(&message[1..], identifier))
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
            "-" => RespIdentifier::RespSimpleErr,
            _ => RespIdentifier::RespSimpleErr,
        }
    }
    // captures the human readable string of the split message
    pub fn serialize_simple(message: &str, identifer: RespIdentifier) -> SerializedMessage {
        let capture_group = Regex::new(r".(.+)\r\n").unwrap();
        let captures = capture_group.captures(message);
    }
    pub fn serialize_int(message: &str) -> SerializedMessage {}
}
