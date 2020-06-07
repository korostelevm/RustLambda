#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;
use std::fmt::{self, Formatter, Display};
use std::error::Error;
use std::fmt::Debug;
// use serde_json::json;
// use serde_json::Result;
use serde_json::json;


// #[derive(Default)]
#[derive(Deserialize, Clone, Debug)]
struct CustomEvent {}
// struct CustomEvent {
//     #[serde(rename = "firstName")]
//     first_name: String,
// }

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: serde_json::Value, c: lambda::Context) -> Result<serde_json::Value, HandlerError> {
    // let j = serde_json::to_string(&e)?;

    // info!("{:#?}", serde_json::to_string(&e)?);
    info!("{:#?}", e);
    // info!("{:?}", e);
    // if e.first_name == "" {
    //     error!("Empty first name in request {}", c.aws_request_id);
    //     return Err(c.new_error("Empty first name"));
    // }
    let res = json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": e.to_string()
    });

    Ok(res)
    // Ok(CustomOutput {
    //     message: format!("Hello, {}!", "sadf"),
    //     // message: format!("Hello, {}!", e.first_name),
    // })
}