use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

// from and not try_from since any query string in technically valid

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
      let mut data = HashMap::new();

      // split query on '&'
      for sub_str in s.split('&') {
          let mut key = sub_str;
          let mut val = "";

          // if we find an '=' 
          if let Some(i) = sub_str.find('=') {
              // assign key to everything before '='
              key = &sub_str[..i];
              // assign val to everything after '='
              val = &sub_str[i + 1..];
          }

          data.entry(key)
              // if we have a value for the key in our hashmap
              .and_modify(|existing: &mut Value| match existing {
                  // if we have a single value, we want to convert it to a multiple value
                  Value::Single(prev_val) => {
                      *existing = Value::Multiple(vec![prev_val, val]);
                  }
                  // if we already have a multiple value, we want to add the new value to the vector
                  Value::Multiple(vec) => vec.push(val),
              })
              // if we dont have a value for this key yet, insert it as a Single
              .or_insert(Value::Single(val));
      }

      QueryString { data }
  }
}