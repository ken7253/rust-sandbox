#[derive(Debug)]
pub struct Http {
    pub method: String,
    pub request_target: String,
    pub protocol: String,
    pub fields: Vec<(String, String)>,
    pub body: String,
}

enum State {
    BeforeMethod,
    InMethod,
    BeforeProtocol,
    InProtocol,
    BeforeRequestTarget,
    InRequestTarget,
    BeforeField,
    InFieldProperty,
    InFieldValue,
    InBody,
}

impl Http {
    pub fn new() -> Self {
        Self {
            method: "".to_string(),
            request_target: "".to_string(),
            protocol: "".to_string(),
            fields: Vec::new(),
            body: "".to_string(),
        }
    }

    pub fn parse(&mut self, data: &String) -> Self {
        let mut state = State::BeforeMethod;
        let mut text_temp = String::new();
        let mut field_temp = (String::new(), String::new());
        let mut new_self = Http::new();

        for char in data.chars() {
            let _ = match state {
                State::BeforeMethod => {
                    text_temp.push(char);
                    state = State::InMethod;
                }
                State::InMethod => {
                    if char == ' ' {
                        new_self.method = text_temp.clone();
                        text_temp.clear();
                        state = State::BeforeRequestTarget;
                    } else {
                        text_temp.push(char);
                    }
                }
                State::BeforeRequestTarget => {
                    if char != ' ' {
                        text_temp.push(char);
                        state = State::InRequestTarget;
                    }
                }
                State::InRequestTarget => {
                    if char != ' ' {
                        text_temp.push(char);
                    } else {
                        new_self.request_target = text_temp.clone();
                        text_temp.clear();
                        state = State::BeforeProtocol
                    }
                }
                State::BeforeProtocol => {
                    if char.is_alphanumeric() && char != ' ' {
                        text_temp.push(char);
                        state = State::InProtocol;
                    }
                }
                State::InProtocol => {
                    if char == '\n' {
                        new_self.protocol = text_temp.clone();
                        text_temp.clear();
                        state = State::BeforeField;
                    } else if char.is_ascii() {
                        text_temp.push(char);
                    }
                }
                State::BeforeField => {
                    if char.is_ascii() {
                        field_temp.0.push(char);
                        state = State::InFieldProperty
                    }
                    if char == '\n' {
                        state = State::InBody;
                    }
                }
                State::InFieldProperty => {
                    if char == ':' {
                        state = State::InFieldValue;
                    } else {
                        field_temp.0.push(char);
                    }
                }
                State::InFieldValue => {
                    if char == '\n' {
                        new_self.fields.push(field_temp);
                        field_temp = (String::new(), String::new());
                        state = State::BeforeField;
                    } else if char != ' ' {
                        field_temp.1.push(char);
                    }
                }
                State::InBody => {
                    text_temp.push(char);
                }
            };
        }

        new_self.body = text_temp.clone();
        text_temp.clear();

        return new_self;
    }
}

#[cfg(test)]
mod tests {
    use crate::http::Http;

    #[test]
    fn parse_method() {
        let mut http = Http::new();
        let parsed =
            http.parse(&"GET / HTTP/1.1\nHost: 127.0.0.1:8880\nUser-Agent: curl/8.5.0".to_string());

        assert_eq!(parsed.method, "GET".to_string());
    }

    #[test]
    fn parse_request_target() {
        let mut http = Http::new();
        let parsed =
            http.parse(&"GET / HTTP/1.1\nHost: 127.0.0.1:8880\nUser-Agent: curl/8.5.0".to_string());

        assert_eq!(parsed.request_target, "/");
    }

    #[test]
    fn parse_protocol() {
        let mut http = Http::new();
        let parsed =
            http.parse(&"GET / HTTP/1.1\nHost: 127.0.0.1:8880\nUser-Agent: curl/8.5.0".to_string());

        assert_eq!(parsed.protocol, "HTTP/1.1")
    }

    #[test]
    fn parse_fields() {
        let mut http = Http::new();
        let parsed = http.parse(&"GET / HTTP/1.1\nHost: 127.0.0.1:8880\n".to_string());

        assert_eq!(parsed.fields[0].0, "Host".to_string());
        assert_eq!(parsed.fields[0].1, "127.0.0.1:8880".to_string());
    }

    #[test]
    fn parse_body() {
        let mut http = Http::new();
        let parsed = http.parse(
            &"GET / HTTP/1.1\nHost: 127.0.0.1:8880\nUser-Agent: curl/8.5.0\n\n{foo: 'bar'}"
                .to_string(),
        );

        assert_eq!(parsed.body, "{foo: 'bar'}");
    }
}
