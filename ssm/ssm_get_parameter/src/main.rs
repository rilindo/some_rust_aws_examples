extern crate clap;
use clap::{Arg, App};

extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{Ssm,
    SsmClient,
    GetParameterRequest,
    GetParameterResult
};

async fn get_parameter(client: &SsmClient, name: &str) -> GetParameterResult {

    let get_parameter_request = GetParameterRequest {
        name: name.to_string(),
        ..Default::default()
    };

    let resp = client.get_parameter(get_parameter_request).await;
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

    let matches = App::new("Example Parameter Get Request Using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Get Parameter")
                            .arg(Arg::with_name("PARAM")
                               .help("Parameter Name")
                               .required(true)
                               .index(1))
                              .get_matches();

    let name = matches.value_of("PARAM").unwrap();
    let client = SsmClient::new(Region::default());
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let resp = rt.block_on(get_parameter(&client, &name));

    process_get_parameter(resp.clone());

}
