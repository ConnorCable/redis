#[allow(dead_code)]
pub mod resp {
    use regex::Regex;
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
        pub length: u32,
        pub identifier: RespIdentifier,
        pub message: String,
    }
    
    fn ret_err_message(error: &str) -> SerializedContainer {
    
        let error_message = SerializedMessage {
            length : error.len() as u32,
            identifier : RespIdentifier::RespSimpleErr,
            message : error.to_string()
        };

        let container = SerializedContainer {
            length : 1,
            messages: vec![error_message]
        };

        return container
    }

    pub fn serialize_array(message: &str, array_length: u32, ) -> SerializedContainer {
        let container = SerializedContainer {
            length: array_length,
            messages : Vec::new(),
        };

        let mut split: VecDeque<&str> = message.split("\r\n").filter(|x| !x.is_empty()).collect();

        while !split.is_empty() {
            if split.len() == 1 {
                ret_err_message("Unpaired identifier / message");
            }
            let pair = split.pop_front().unwrap().split("");
            let Some(message_identifier) = pair.get(0) {
                
            }
            let entry = SerializedMessage {
                identifier : detect_identifier(split.pop_front().unwrap()).unwrap(),
                message: split.pop_front().unwrap().to_string(),
                length: 
            };
        }


        println!("{:?}", split);

        return container

    }

    pub fn construct_serialized_message(message: &str) -> SerializedContainer {
        let mut split: Vec<&str> = message.split("").filter(|x| !x.is_empty()).collect();
        let mut container = SerializedContainer {
            length: 0,
            messages: Vec::new(),
        };
        let Some(id) = detect_identifier(split.remove(0)) else {
            println!("Could not detect identifier");
            return ret_err_message("Could not detect identifier")
        };

        let Some(extracted) = process_resp_simple(message) else {
            println!("Could not process simple message");
            return ret_err_message("Could not process simple message")
        };

        let serializedmessage = SerializedMessage {
            length: extracted.len() as u32,
            identifier: id,
            message: extracted.to_string(),
        };

        container.messages.push(serializedmessage);
        container.length += 1;
        return container;
    }

    fn detect_identifier(c: &str) -> Option<RespIdentifier> {
        match c {
            "+" => Some(RespIdentifier::RespSimpleStr),
            "-" => Some(RespIdentifier::RespSimpleErr),
            ":" => Some(RespIdentifier::RespInt),
            "#" => Some(RespIdentifier::RespBool),
            "*" => Some(RespIdentifier::RespArray),
            "$" => Some(RespIdentifier::RespBulkStr),
            _ => Some(RespIdentifier::RespSimpleErr),
        }
    }
    // captures the human readable string of the split message
    pub fn process_resp_simple(message: &str) -> Option<&str> {
        let capture_group = Regex::new(r".(.+)\r\n").unwrap();
        let captures = capture_group.captures(message);
        match captures {
            Some(x) => return Some(x.get(1).unwrap().as_str()),
            None => return None,
        }
    }
}
