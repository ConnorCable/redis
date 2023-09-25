mod serialize;

#[cfg(test)]
mod serialize_tests {
    use crate::serialize::resp::{
        construct_serialized_message, RespIdentifier, SerializedContainer, SerializedMessage,
    };

    fn construct_and_fill_test_container(input: String, id: RespIdentifier) -> SerializedContainer {
        let serialized = SerializedMessage {
            length: 1,
            identifier: id,
            message: input,
        };
        let container = SerializedContainer {
            length: 1,
            messages: vec![serialized],
        };

        return container;
    }

    fn construct_and_fill_test_container_with_array(
        input: Vec<String>,
        id_vector: Vec<RespIdentifier>,
    ) -> SerializedContainer {
        let mut serialized_vectors: Vec<SerializedMessage> = Vec::new();

        for i in 0..input.len() {
            let data = input[i].clone();
            let serialized = SerializedMessage {
                length: data.len(),
                identifier: id_vector[i],
                message: data,
            };
            serialized_vectors.push(serialized);
        }

        let container = SerializedContainer {
            length: input.len() as u32,
            messages: serialized_vectors,
        };

        return container;
    }

    #[test]
    fn test_serialize_simplestr() {
        let message = String::from("+OK\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container =
            construct_and_fill_test_container("OK".to_string(), RespIdentifier::RespSimpleStr);

        let first_compare = test_container.messages;
        let second_compare = serialized.messages;

        for (item1, item2) in first_compare.iter().zip(second_compare.iter()) {
            assert_eq!(
                item1.message, item2.message,
                "Compare messages in test container vs functionally created container"
            )
        }
    }
    #[test]
    fn test_serialize_int() {
        let message = String::from(":1\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container =
            construct_and_fill_test_container("1".to_string(), RespIdentifier::RespInt);

        let first_compare = test_container.messages;
        let second_compare = serialized.messages;

        for (item1, item2) in first_compare.iter().zip(second_compare.iter()) {
            assert_eq!(
                item1.message, item2.message,
                "Compare messages in test container vs functionally created container"
            )
        }
    }
    #[test]
    fn test_serialize_simple_error() {
        let message = String::from("-ERR\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container =
            construct_and_fill_test_container("ERR".to_string(), RespIdentifier::RespSimpleErr);

        let first_compare = test_container.messages;
        let second_compare = serialized.messages;

        for (item1, item2) in first_compare.iter().zip(second_compare.iter()) {
            assert_eq!(
                item1.message, item2.message,
                "Compare messages in test container vs functionally created container"
            )
        }
    }
    #[test]
    fn test_invalid_message() {
        let message = String::from("71\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        assert_eq!(
            serialized.messages[0].identifier,
            RespIdentifier::RespSimpleErr,
            "Testing if identifier is correct"
        );
    }
    #[test]
    fn test_empty_bulk_string() { 
        let message = String::from("$0\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container = construct_and_fill_test_container("".to_string(), RespIdentifier::RespBulkStr);
        assert_eq!(serialized.messages[0].message, test_container.messages[0].message);
        assert_eq!(serialized.messages[0].identifier, test_container.messages[0].identifier);
    }

    fn test_null_bulk_string() { 
        let message = String::from("$-1\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container = construct_and_fill_test_container("".to_string(), RespIdentifier::RespBulkStr);
        assert_eq!(serialized.messages[0].message, test_container.messages[0].message);
        assert_eq!(serialized.messages[0].identifier, test_container.messages[0].identifier);
    }


    #[test]
    fn test_resp_array() {
        let message = "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n";
        let message_vector = vec![String::from("echo"), String::from("hello world")];
        let identifier_vector = vec![RespIdentifier::RespBulkStr, RespIdentifier::RespBulkStr];

        let serialized: SerializedContainer = construct_serialized_message(message);
        let test_container =
            construct_and_fill_test_container_with_array(message_vector, identifier_vector);

        for i in 0..serialized.messages.len() {
            assert_eq!(
                serialized.messages[i].message, test_container.messages[i].message,
                "Compare messages in test container vs functionally created container"
            );
            assert_eq!(
                serialized.messages[i].identifier, test_container.messages[i].identifier,
                "Compare identifiers in test"
            );
        }
    }
    #[test]
    fn test_resp_array_variable_types() {
        let message = "*3\r\n+OK\r\n:1\r\n$4\r\necho\r\n";
        let message_vector = vec![String::from("OK"), String::from("1"), String::from("echo")];
        let identifier_vector = vec![
            RespIdentifier::RespSimpleStr,
            RespIdentifier::RespInt,
            RespIdentifier::RespBulkStr,
        ];

        let serialized: SerializedContainer = construct_serialized_message(message);
        let test_container =
            construct_and_fill_test_container_with_array(message_vector, identifier_vector);

        for i in 0..serialized.messages.len() {
            assert_eq!(
                serialized.messages[i].message, test_container.messages[i].message,
                "Compare messages in test container vs functionally created container"
            );
            assert_eq!(
                serialized.messages[i].identifier, test_container.messages[i].identifier,
                "Compare identifiers in test"
            );
        }
    }
    #[test]
    #[should_panic]
    fn test_invalid_array() {
        let message = "*\r\n$4\r\necho\r\n$11\r\nhello world\r\n";
        let message_vector = vec![String::from("echo"), String::from("hello world")];
        let identifier_vector = vec![RespIdentifier::RespBulkStr, RespIdentifier::RespBulkStr];

        let serialized: SerializedContainer = construct_serialized_message(message);
        let test_container =
            construct_and_fill_test_container_with_array(message_vector, identifier_vector);

        for i in 0..serialized.messages.len() {
            assert_eq!(
                serialized.messages[i].message, test_container.messages[i].message,
                "Compare messages in test container vs functionally created container"
            );
            assert_eq!(
                serialized.messages[i].identifier, test_container.messages[i].identifier,
                "Compare identifiers in test"
            );
        }
    }
}
