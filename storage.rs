use std::io::{Error, ErrorKind};
use std::collections::HashMap;

struct Point {
    key: String,
    val: String
}

fn store_points(a: Vec<Point>, storage: &mut HashMap<String, String>) {
    let mut i: usize = 0;
    while i != a.len() {
        storage.insert(a[i].key.to_string(), a[i].val.to_string());
        i += 1;
    }
}

fn count_char_val(a: char, storage: &HashMap<String, String>) -> u32 {
    let mut c: u32 = 0;
    for (key, val) in storage {
        for b in val.chars() {
            if a == b {
                c += 1;
            }
        }
    }
    return c;
}

fn conc_keys(a: usize, storage: &HashMap<String,String>) -> Result<String, Error> {
    let mut r = "".to_string();
    for (key, val) in storage {
        if a < 0 || a >= key.len() {
            return Err(Error::new(ErrorKind::Other, "Must be a valid string index"));
        }
        r.push_str(&key[..a]);
    }
    return Ok(r.to_string());
}

fn main() {
   let mut storage: HashMap<String, String> = HashMap::new();
   let v = vec![
       Point {
           key: String::from("something something"),
           val: String::from("has no value")
       },
       Point {
           key: String::from(" has"),
           val: String::from(" some a")
       },
       Point {
           key: String::from(" value"),
           val: String::from(" aaaa")
       }
   ];
   store_points(v, &mut storage);
   let r1 = count_char_val('a', &storage);
   let r2 = conc_keys(100, &storage).unwrap_or_else( |error| {
       panic!("Couldnt handle the iob");
   });
   println!("There are {} 'a' characters in values", r1);
   println!("The resultant of concatenating the first 2 characters of all keys is '{}'", r2);
}
