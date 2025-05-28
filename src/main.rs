// Copy & pasted from https://github.com/awslabs/aws-lambda-rust-runtime

use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use serde_json::{json, Value};

use rust_game_review_lambda::event_parsing::parse_event;
use rust_game_review_lambda::run;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let func = service_fn(my_lambda_func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_lambda_func(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let (start_pos, moves) = parse_event(event).expect("API key should match");

    run(start_pos, moves)?;

    // End
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(json!({"message": "success"}))
}

#[tokio::test]
async fn test_my_lambda_handler() {
    let input = serde_json::from_str("{\"body\": \"adf_s_e2e4 e7e5\"}").expect("failed to parse event");
    let context = lambda_runtime::Context::default();

    let event = lambda_runtime::LambdaEvent::new(input, context);

    my_lambda_func(event).await.expect("failed to handle event");
}