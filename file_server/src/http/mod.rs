struct Http {
    method: String,
    request_target: String,
    protocol: String,
    fields: Vec<(String, String)>,
    body: String,
}

enum State {
    BeforeMethod,
    InMethod,
    BeforeProtocol,
    InProtocol,
    BeforeFields,
    InFieldProperty,
    BeforeFieldValue,
    InFieldValue,
    BeforeBody,
    InBody,
}

impl Http {
    fn new() -> Self {
        Self {
            method: "".to_string(),
            request_target: "".to_string(),
            protocol: "".to_string(),
            fields: Vec::new(),
            body: "".to_string(),
        }
    }

    fn parse(&mut self, data: String) -> Self {
        let mut state = State::BeforeMethod;
        let mut text_temp = String::new();
        let mut new_self = Http::new();

        for char in data.chars() {
            match state {
                State::BeforeMethod => {
                    text_temp.push(char);
                    state = State::InMethod;
                }
                State::InMethod => {
                    if char == '/' {
                        new_self.method = text_temp.clone();
                        text_temp.clear();
                        state = State::BeforeProtocol;
                    }
                }
                _ => {}
            }
        }

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
            http.parse("GET / HTTP/1.1\nHost: 127.0.0.1:8880\nUser-Agent: curl/8.5.0".to_string());

        assert_eq!(parsed.method, "GET".to_string());
    }
}
