use std::collections::HashMap;

macro_rules! my_matches {
    ($value:expr, $pattern:expr) => {
        match $value {
            $pattern => true,
            _ => false,
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl From<bool> for Json {
    fn from(boolean: bool) -> Self {
        Json::Boolean(boolean)
    }
}

macro_rules! impl_for_number {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(number: $t) -> Self {
                    Json::Number(number as f64)
                }
            }
        )*
    };
}

impl_for_number!(u8 i8 u16 i16 u32 i32 u64 i64 f32 f64 usize isize i128 u128);

impl From<String> for Json {
    fn from(string: String) -> Self {
        Json::String(string)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(string: &'a str) -> Self {
        Json::String(string.to_string())
    }
}

#[macro_export]
macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        Json::Array(vec![ $( json!($element) ),* ])
    };
    ({ $( $key:tt : $value:tt ),* }) => {
        {
            let mut fields = ::std::collections::HashMap::new();
            $( fields.insert($key.to_string(), json!($value)); )*
            Json::Object(fields)
        }
    };
    ($other:tt) => {
        Json::from($other)
    };
}

macro_rules! setup_stream {
    ($stream:ident, $address:ident) => {
        let $stream = ::std::net::TcpStream::connect($address);
    };
}

#[test]
fn test_req_macro() {
    let address = "127.0.0.1:8080";
    setup_stream!(stream, address);
    assert!(stream.is_err());
}

#[test]
fn test_json_null() {
    let n = json!(null);
    assert_eq!(n, Json::Null);
}

#[test]
fn test_json_array() {
    let array = json!([1, 2, 3]);
    assert_eq!(
        array,
        Json::Array(
            vec![1, 2, 3]
                .iter()
                .map(|n| From::from(n.clone()))
                .collect()
        )
    );
}

#[test]
fn test_json_object() {
    let n = 100_u8;
    let object = json!({
        "a": 1.0, "b": n
    });
    let mut hm = HashMap::new();
    hm.insert("a".to_string(), Json::from(1.0));
    hm.insert("b".to_string(), Json::from(100));
    assert_eq!(object, Json::Object(hm));
}

#[test]
fn test_matches() {
    let closure = || 1200;
    assert!(my_matches!(closure(), 1200));
    assert_eq!(my_matches!(closure(), 200), false);
}