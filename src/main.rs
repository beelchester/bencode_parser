use std::env;

const DECODE: &str = "decode";
const INT_PREFIX: &str = "i";
const LIST_PREFIX: &str = "l";
const DICT_PREFIX: &str = "d";
const SUFFIX: &str = "e";

fn decode(val: &str) -> (serde_json::Value, &str) {
    match val.split_at(1) {
        // integer
        (INT_PREFIX, rest) => {
            if let Some((int, rest)) = rest.split_once(SUFFIX) {
                if let Ok(val) = int.parse::<i64>() {
                    return (serde_json::Value::Number(val.into()), rest);
                }
            }
        }
        // list
        (LIST_PREFIX, rest) => {
            let (list, rest) = get_list_of_values(rest);
            return (serde_json::Value::Array(list), rest);
        }
        // dictionary
        (DICT_PREFIX, rest) => {
            if let Some(val) = rest.strip_suffix(SUFFIX) {
                let (list, rest) = get_list_of_values(val);
                // convert list to dictionary
                let mut dict = serde_json::Map::new();
                list.iter().enumerate().for_each(|(i, value)| {
                    if i % 2 == 0 {
                        // key should always be a string
                        if let serde_json::Value::String(key) = value {
                            if list.len() > i + 1 {
                                dict.insert(key.to_string(), list[i + 1].clone());
                            } else {
                                panic!("Value not provided");
                            }
                        }
                    }
                });
                return (serde_json::Value::Object(dict), rest);
            }
        }
        (_, _) => {
            // byte string
            if let Some((len, rest)) = val.split_once(':') {
                if let Ok(len) = len.parse::<usize>() {
                    return (
                        serde_json::Value::String(rest[..len].to_string()),
                        &rest[len..],
                    );
                }
            }
        }
    }
    panic!("Unknown argument");
}

fn get_list_of_values(val: &str) -> (Vec<serde_json::Value>, &str) {
    let mut list = vec![];
    let mut rest = val;
    while !rest.is_empty() && !rest.starts_with('e') {
        let (decoded, r) = decode(rest);
        list.push(decoded.clone());
        rest = r;
    }
    if rest.starts_with('e') {
        rest = &rest[1..];
    }
    (list, rest)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == DECODE {
        if args.len() >= 3 {
            let encoded = &args[2];
            let (decoded, _) = decode(encoded);
            println!("{}", decoded)
        } else {
            println!("Decode command requires an argument");
        }
    } else {
        println!("Unknown command: {}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_decode_int() {
        let (decoded, _) = decode("i123e");
        assert_eq!(decoded, serde_json::Value::Number(123.into()));
    }

    #[test]
    fn test_decode_byte_string() {
        let (decoded, _) = decode("4:spam");
        assert_eq!(decoded, serde_json::Value::String("spam".to_string()));
    }

    #[test]
    fn test_decode_list() {
        let (decoded, _) = decode("l5:helloi52e2:hil1:bed4:test2:okee");
        let mut map = serde_json::Map::new();
        map.insert(
            "test".to_string(),
            serde_json::Value::String("ok".to_string()),
        );
        assert_eq!(
            decoded,
            serde_json::Value::Array(vec![
                serde_json::Value::String("hello".to_string()),
                serde_json::Value::Number(52.into()),
                serde_json::Value::String("hi".to_string()),
                serde_json::Value::Array(vec![serde_json::Value::String("b".to_string())]),
                serde_json::Value::Object(map)
            ])
        );
    }

    #[test]
    fn test_decode_dict() {
        let (decoded, _) = decode("d3:foo3:bar5:helloi52e2:nod2:hi2:byee");
        let mut map = serde_json::Map::new();
        let mut nested_map = serde_json::Map::new();
        map.insert(
            "foo".to_string(),
            serde_json::Value::String("bar".to_string()),
        );
        map.insert("hello".to_string(), serde_json::Value::Number(52.into()));
        nested_map.insert(
            "hi".to_string(),
            serde_json::Value::String("by".to_string()),
        );
        map.insert("no".to_string(), serde_json::Value::Object(nested_map));
        assert_eq!(decoded, serde_json::Value::Object(map));
    }
}
