extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{Ssm,
    SsmClient,
    GetParameterRequest,
    GetParameterResult
};

async fn get_parameter(client: &SsmClient, name: &str) -> GetParameterResult {

    let get_parameter_req = GetParameterRequest {
        name: name.to_string(),
        ..Default::default()
    };

    let resp = client.get_parameter(get_parameter_req).await;
    return resp.unwrap()
}

fn process_get_parameter(resp: GetParameterResult) {

    match resp.parameter {
        Some(parameter) => {
            match parameter.name {
                Some(name) => println!("Parameter Name: {}", name),
                None => println!("No parameter name"),
            }
            match parameter.arn {
                Some(arn) => println!("Parameter ARN: {}", arn),
                None => println!("no arn"),
            }
            match parameter.value {
                Some(value) => println!("Parameter Value: {}", value),
                None => println!("no arn"),
            }
        }
        None => println!("No tokens available"),
    }
}


fn main() {

    let client = SsmClient::new(Region::default());
    let name = "/test/parameter";

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(get_parameter(&client, &name));

    process_get_parameter(resp.clone());


}
