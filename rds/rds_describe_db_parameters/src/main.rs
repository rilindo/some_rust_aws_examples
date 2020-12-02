extern crate rusoto_core;
extern crate rusoto_rds;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_rds::{DBParameterGroupDetails, DescribeDBParametersMessage, Rds, RdsClient};

async fn describe_db_parameters_request(
    client: &RdsClient,
    describe_db_parameters_req: DescribeDBParametersMessage,
) -> DBParameterGroupDetails {
    let resp = client
        .describe_db_parameters(describe_db_parameters_req)
        .await;
    return resp.unwrap();
}

fn describe_db_parameters_result(resp: DBParameterGroupDetails) {
    for param in resp.parameters.iter() {
        for p in param.iter() {
            match p.clone().parameter_value {
                Some(parameter_value) => {
                    println!(
                        "{}={}",
                        p.parameter_name.clone().unwrap(),
                        parameter_value.clone()
                    )
                }
                None => {
                    println!("{}=unset", p.parameter_name.clone().unwrap())
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("Example of making a db paramegter call using Rust and Rusoto")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Describe Instances")
        .arg(
            Arg::with_name("param")
                .short("p")
                .long("parameter")
                .help("parameter name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let param = matches.value_of("param").unwrap();
    let client = RdsClient::new(Region::default());
    let describe_db_parameters_req = DescribeDBParametersMessage {
        db_parameter_group_name: param.clone().to_string(),
        marker: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_db_parameters_request(
        &client,
        describe_db_parameters_req,
    ));

    describe_db_parameters_result(resp.clone());

    while resp.clone().marker != None {
        let describe_db_parameters_req = DescribeDBParametersMessage {
            db_parameter_group_name: param.clone().to_string(),
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(describe_db_parameters_request(
            &client,
            describe_db_parameters_req,
        ));
        describe_db_parameters_result(resp.clone());
    }
}
