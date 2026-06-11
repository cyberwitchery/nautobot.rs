use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    match value {
        serde_json::Value::Object(object) => {
            let mut params = vec![];
            for (key, value) in object {
                match value {
                    serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                        &format!("{}[{}]", prefix, key),
                        value,
                    )),
                    serde_json::Value::Array(array) => {
                        for (i, value) in array.iter().enumerate() {
                            params.append(&mut parse_deep_object(
                                &format!("{}[{}][{}]", prefix, key, i),
                                value,
                            ));
                        }
                    }
                    serde_json::Value::String(s) => {
                        params.push((format!("{}[{}]", prefix, key), s.clone()))
                    }
                    _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
                }
            }
            params
        }
        serde_json::Value::Array(array) => {
            let mut params = vec![];
            for (i, value) in array.iter().enumerate() {
                params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, i),
                    value,
                ));
            }
            params
        }
        serde_json::Value::String(s) => vec![(prefix.to_string(), s.clone())],
        serde_json::Value::Null => vec![],
        _ => vec![(prefix.to_string(), value.to_string())],
    }
}

#[cfg(not(docsrs))]
pub mod circuits_api;
#[cfg(not(docsrs))]
pub mod cloud_api;
#[cfg(not(docsrs))]
pub mod core_api;
#[cfg(not(docsrs))]
pub mod data_validation_api;
#[cfg(not(docsrs))]
pub mod dcim_api;
#[cfg(not(docsrs))]
pub mod extras_api;
#[cfg(not(docsrs))]
pub mod graphql_api;
#[cfg(not(docsrs))]
pub mod ipam_api;
#[cfg(not(docsrs))]
pub mod load_balancers_api;
#[cfg(not(docsrs))]
pub mod status_api;
#[cfg(not(docsrs))]
pub mod swagger_api;
#[cfg(not(docsrs))]
pub mod swagger_json_api;
#[cfg(not(docsrs))]
pub mod swagger_yaml_api;
#[cfg(not(docsrs))]
pub mod tenancy_api;
#[cfg(not(docsrs))]
pub mod ui_api;
#[cfg(not(docsrs))]
pub mod users_api;
#[cfg(not(docsrs))]
pub mod virtualization_api;
#[cfg(not(docsrs))]
pub mod vpn_api;
#[cfg(not(docsrs))]
pub mod wireless_api;

pub mod configuration;

#[cfg(test)]
mod tests {
    use super::parse_deep_object;
    use serde_json::json;

    #[test]
    fn object_with_string_values() {
        let value = json!({"name": "alice", "role": "admin"});
        let mut result = parse_deep_object("filter", &value);
        result.sort();
        assert_eq!(
            result,
            vec![
                ("filter[name]".to_string(), "alice".to_string()),
                ("filter[role]".to_string(), "admin".to_string()),
            ]
        );
    }

    #[test]
    fn nested_object() {
        let value = json!({"outer": {"inner": "val"}});
        let result = parse_deep_object("q", &value);
        assert_eq!(
            result,
            vec![("q[outer][inner]".to_string(), "val".to_string())]
        );
    }

    #[test]
    fn object_with_number_and_bool() {
        let value = json!({"count": 42, "active": true});
        let mut result = parse_deep_object("p", &value);
        result.sort();
        assert_eq!(
            result,
            vec![
                ("p[active]".to_string(), "true".to_string()),
                ("p[count]".to_string(), "42".to_string()),
            ]
        );
    }

    #[test]
    fn object_with_null_value() {
        let value = json!({"key": null});
        let result = parse_deep_object("p", &value);
        // null fields inside an object use to_string() via the catch-all arm
        assert_eq!(
            result,
            vec![("p[key]".to_string(), "null".to_string())]
        );
    }

    #[test]
    fn object_with_array_of_strings() {
        let value = json!({"tags": ["a", "b"]});
        let result = parse_deep_object("f", &value);
        assert_eq!(
            result,
            vec![
                ("f[tags][0]".to_string(), "a".to_string()),
                ("f[tags][1]".to_string(), "b".to_string()),
            ]
        );
    }

    #[test]
    fn object_with_array_of_objects() {
        let value = json!({"items": [{"id": 1}, {"id": 2}]});
        let result = parse_deep_object("f", &value);
        assert_eq!(
            result,
            vec![
                ("f[items][0][id]".to_string(), "1".to_string()),
                ("f[items][1][id]".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn top_level_string() {
        let value = json!("hello");
        let result = parse_deep_object("key", &value);
        assert_eq!(result, vec![("key".to_string(), "hello".to_string())]);
    }

    #[test]
    fn top_level_number() {
        let value = json!(99);
        let result = parse_deep_object("key", &value);
        assert_eq!(result, vec![("key".to_string(), "99".to_string())]);
    }

    #[test]
    fn top_level_bool() {
        let value = json!(false);
        let result = parse_deep_object("key", &value);
        assert_eq!(result, vec![("key".to_string(), "false".to_string())]);
    }

    #[test]
    fn top_level_null() {
        let value = json!(null);
        let result = parse_deep_object("key", &value);
        assert_eq!(result, Vec::<(String, String)>::new());
    }

    #[test]
    fn top_level_array() {
        let value = json!(["x", "y"]);
        let result = parse_deep_object("arr", &value);
        assert_eq!(
            result,
            vec![
                ("arr[0]".to_string(), "x".to_string()),
                ("arr[1]".to_string(), "y".to_string()),
            ]
        );
    }

    #[test]
    fn empty_object() {
        let value = json!({});
        let result = parse_deep_object("p", &value);
        assert_eq!(result, Vec::<(String, String)>::new());
    }

    #[test]
    fn empty_array() {
        let value = json!([]);
        let result = parse_deep_object("p", &value);
        assert_eq!(result, Vec::<(String, String)>::new());
    }

    #[test]
    fn deeply_nested() {
        let value = json!({"a": {"b": {"c": "deep"}}});
        let result = parse_deep_object("q", &value);
        assert_eq!(
            result,
            vec![("q[a][b][c]".to_string(), "deep".to_string())]
        );
    }

    #[test]
    fn mixed_array_elements() {
        let value = json!({"mix": [1, "two", true, null, {"k": "v"}]});
        let result = parse_deep_object("f", &value);
        assert_eq!(
            result,
            vec![
                ("f[mix][0]".to_string(), "1".to_string()),
                ("f[mix][1]".to_string(), "two".to_string()),
                ("f[mix][2]".to_string(), "true".to_string()),
                // null elements produce no output
                ("f[mix][4][k]".to_string(), "v".to_string()),
            ]
        );
    }
}
