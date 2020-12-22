use serde_json::json;

pub trait Writer {
    fn message(&self, data: &str);
    fn output(&self, data: &str);
}

pub struct JSONWriter;

impl Writer for JSONWriter {
    fn message(&self, _data: &str) {}

    fn output(&self, data: &str) {
        println!("{}", json!({
            "type": "success",
            "result": data,
        }));
    }
}

pub struct CLIWriter;

impl Writer for CLIWriter {
    fn message(&self, data: &str) {
        println!("{}", data);
    }

    fn output(&self, data: &str) {
        println!("{}", data);
    }
}

pub fn get_writer(json_output: bool) -> Box<dyn Writer> {
    if json_output {
        Box::new(JSONWriter)
    } else {
        Box::new(CLIWriter)
    }
}