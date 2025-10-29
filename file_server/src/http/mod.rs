struct Http {
    method: String,
    request_target: String,
    protocol: String,
    header: Vec<(String, String)>,
    body: String,
}

impl Http {
    fn new() -> Self {
        Self {
            method: "".to_string(),
            request_target: "".to_string(),
            protocol: "".to_string(),
            header: Vec::new(),
            body: "".to_string(),
        }
    }

    fn parse(&mut self, data: String) -> Self {
        data.split("\n");

        todo!();
    }
}
