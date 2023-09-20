mod serialize;




#[cfg(test)]
mod serialize_tests {
    use crate::serialize::resp::{SerializedMessage, RespIdentifier, construct_serialized_message, SerializedContainer, serialize_array};

    fn construct_and_fill_test_container(input : String, id : RespIdentifier) -> SerializedContainer {
        let serialized = SerializedMessage { length: 1, identifier: id, message: input };
        let container = SerializedContainer {
            length: 1,
            messages: vec![serialized]
        };

        return container;
    }

    #[test]
    fn test_serialize_simplestr() {
        let message = String::from("+OK\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container = construct_and_fill_test_container("OK".to_string(), RespIdentifier::RespSimpleStr);


        let first_compare = test_container.messages;
        let second_compare = serialized.messages;

        for(item1,item2) in first_compare.iter().zip(second_compare.iter()) {
            assert_eq!(item1.message, item2.message , "Compare messages in test container vs functionally created container")
        }

        
        
    }
    #[test]
    fn test_serialize_int() {
        let message = String::from(":1\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container = construct_and_fill_test_container("1".to_string(), RespIdentifier::RespInt);


        let first_compare = test_container.messages;
        let second_compare = serialized.messages;

        for(item1,item2) in first_compare.iter().zip(second_compare.iter()) {
            assert_eq!(item1.message, item2.message , "Compare messages in test container vs functionally created container")
        }
    }
    #[test]
    fn test_serialize_simple_error() {
        let message = String::from("-ERR\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        let test_container = construct_and_fill_test_container("ERR".to_string(), RespIdentifier::RespSimpleErr);

        let first_compare = test_container.messages;
        let second_compare = serialized.messages;

        for(item1,item2) in first_compare.iter().zip(second_compare.iter()) {
            assert_eq!(item1.message, item2.message , "Compare messages in test container vs functionally created container")
        }

    }
    #[test]
    fn test_invalid_message() {
         let message = String::from("71\r\n");
        let serialized: SerializedContainer = construct_serialized_message(&message);
        assert_eq!(serialized.messages[0].identifier , RespIdentifier::RespSimpleErr, "Testing if identifier is correct");

    }
    #[test]
    fn test_resp_array() {
        let message = "\r\n$4\r\necho\r\n$5\r\nhello world\r\n";

       serialize_array(message, 2);


    }

}