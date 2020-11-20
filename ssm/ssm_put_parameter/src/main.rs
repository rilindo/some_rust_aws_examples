extern crate clap;
use clap::{Arg, App};

extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{Ssm,
    SsmClient,
    PutParameterRequest,
    PutParameterResult
};

async fn put_parameter(client: &SsmClient, name: &str, value: &str) -> PutParameterResult {

    let put_parameter_request  = PutParameterRequest {
        name: name.clone().to_string(),
        value: value.clone().to_string(),
        data_type: Some("text".to_string()),
        type_: Some("String".to_string()),
        overwrite: Some(true),
        ..Default::default()
    };

    let resp = client.put_parameter(put_parameter_request).await;
    return resp.unwrap()
}

fn process_put_parameter(resp: PutParameterResult) {

    match resp.version {
        Some(version) => println!("Version: {}", version),
        None => println!("Unable to insert parameter"),
    }
}

fn main() {

    let matches = App::new("Example Parameter Put Request Using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Insert New Parameter")
                            .arg(Arg::with_name("NAME")
                               .help("Parameter Name")
                               .required(true)
                               .index(1))
                            .arg(Arg::with_name("VALUE")
                                .help("Parameter Value")
                                .required(true)
                                .index(2))
                              .get_matches();

    let name = matches.value_of("NAME").unwrap();
    let value = matches.value_of("VALUE").unwrap();


    let client = SsmClient::new(Region::default());

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let resp = rt.block_on(put_parameter(&client, &name, &value));

    process_put_parameter(resp.clone());
}
