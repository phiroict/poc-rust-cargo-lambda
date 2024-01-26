use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
mod response;
use response::ResponseType;
use serde_json;
use chrono::Local;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let param_where = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("where"))
        .unwrap_or("anywhere");
    let timestamp = Local::now().format("%Y%m%d%H%M%S-%s");
    let json_object = ResponseType {
        name: who.to_string(),
        time: timestamp.to_string(),
        payload: format!("Hello {who} at {param_where} this is an json response")
    };
    let message = serde_json::to_string(&json_object).unwrap_or("None".to_string());

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
