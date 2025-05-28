use lambda_runtime::LambdaEvent;
use serde_json::Value;

const API_KEY: &'static str = "nfY4b672";

pub fn parse_event(event: LambdaEvent<Value>) -> Option<(String, String)>  {
    let (event, _context) = event.into_parts();

    let body = event["body"].as_str().expect("lambda event should have a field named body");
    let args: Vec<&str> = body.split("_").collect();

    let api_key = args[0];
    let start_pos = args[1].to_string();
    let moves = args[2].to_string();

    if api_key != API_KEY {
        // return None
    }

    Some((start_pos, moves))
}