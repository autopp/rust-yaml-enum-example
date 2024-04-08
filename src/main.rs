use std::io::{stdin, Read};

use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Hash, PartialEq, Eq, Debug)]
enum Method {
    #[serde(alias = "GET", alias = "get")]
    Get,
    #[serde(alias = "POST", alias = "post")]
    Post,
    #[serde(alias = "PUT", alias = "put")]
    Put,
    #[serde(alias = "PATCH", alias = "patch")]
    Patch,
    #[serde(alias = "DELETE", alias = "delete")]
    Delete,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Response {
    pub status: u16,
    pub body: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Config {
    pub paths: IndexMap<String, IndexMap<Method, Response>>,
}

fn parse_config(source: &str) -> Result<Config, String> {
    serde_yaml::from_str(source).map_err(|e| e.to_string())
}

fn main() {
    let mut source = String::new();
    stdin().read_to_string(&mut source).unwrap();
    println!("{:?}", parse_config(&source));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::indexmap;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_config() {
        let source = r#"
paths:
    /hello:
        get:
            status: 200
            body: Hello, world!
        "#;
        let expected = Ok(Config {
            paths: indexmap! {
                "/hello".to_string() => indexmap! {
                    Method::Get => Response {
                        status: 200,
                        body: "Hello, world!".to_string(),
                    },
                },
            },
        });
        let actual = parse_config(source);

        assert_eq!(expected, actual);
    }
}
