use std::env;

fn decode(val: &str) -> serde_json::Value {
    // integer
    // i3e
    if val.starts_with('i') {
        if let Some(val) = val.strip_prefix('i') {
            if let Some(ind) = val.find('e') {
                let temp = &val[..ind];
                if let Ok(val) = temp.parse::<i64>() {
                    return serde_json::Value::Number(serde_json::Number::from(val));
                }
            }
        }
    }
    // byte string
    // 4:spam
    if let Some((len, rest)) = val.split_once(':') {
        if let Ok(len) = len.parse::<usize>() {
            return serde_json::Value::String(rest[..len].to_string());
        }
    }
    // list
    // l4:spam4:eggse
    if val.starts_with('l') {
        if let Some(val) = val.strip_prefix('l') {
            if let Some(val) = val.strip_suffix('e') {
                let list = get_list_of_values(val);
                return serde_json::Value::Array(list);
            }
        }
    }
    // dictionary
    // d3:cow3:moo4:spam4:eggse
    if val.starts_with('d') {
        if let Some(val) = val.strip_prefix('d') {
            if let Some(val) = val.strip_suffix('e') {
                let list = get_list_of_values(val);
                // convert list to dictionary
                let mut dict = serde_json::Map::new();
                list.iter().enumerate().for_each(|(i, value)| {
                    if i % 2 == 0 {
                        // key should always be a string
                        if let serde_json::Value::String(key) = value {
                            dict.insert(key.to_string(), list[i + 1].clone());
                        }
                    }
                });
                return serde_json::Value::Object(dict);
            }
        }
    }
    serde_json::Value::String("na".to_string())
}

fn get_list_of_values(val: &str) -> Vec<serde_json::Value> {
    let mut list = vec![];
    let mut rest = val;
    while !rest.is_empty() {
        let decoded = decode(rest);
        list.push(decoded.clone());
        rest = &rest[decoded.to_string().trim_matches('\"').len() + 2..];
    }
    list
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        if args.len() >= 3 {
            let encoded = &args[2];
            let decoded = decode(encoded);
            println!("{}", decoded)
        } else {
            println!("decode command requires an argument");
        }
    } else {
        println!("unknown command: {}", args[1]);
    }
}
