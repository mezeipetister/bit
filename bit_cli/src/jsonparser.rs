// use std::collections::HashMap;

// pub struct Items {
//     dates: Vec<String>,
//     accounts: HashMap<String, Vec<i32>>,
// }

// pub fn to_string(items: Items) -> String {
//     // Serialize it to a JSON string.
//     let json_string = match serde_json::to_string(&items) {
//         Ok(json) => json,
//         Err(_msg) => panic!("Error while parsing data into JSON"),
//     };
//     return json_string;
// }
