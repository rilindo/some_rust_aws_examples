extern crate rusoto_core;
extern crate rusoto_cloudfront;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_cloudfront::{CloudFront,
    CloudFrontClient,
    GetDistributionRequest,
    GetDistributionResult,
};

async fn get_distribution(client: &CloudFrontClient, get_distribution_request: GetDistributionRequest) -> GetDistributionResult {

    let resp = client.get_distribution(get_distribution_request).await;
    return resp.unwrap();

}

fn get_distribution_result(resp: GetDistributionResult) {

    match resp.distribution {
        Some(distribution) => {
            println!("Distribution ARN:{}\n Domain Name: {}", distribution.arn, distribution.domain_name)
        }
        None => println!("Unable to retrieve distribution information")
    }
}

fn main() {

    let matches = App::new("Example Get Distributions call using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Get Distributions")
                            .arg(Arg::with_name("distribution_id")
                                .short("d")
                                .long("distribution_id")
                                .help("Set Distribution ID")
                                .required(true)
                                .takes_value(true))
                              .get_matches();

    let distribution_id = matches.value_of("distribution_id").unwrap().to_string();

    let client = CloudFrontClient::new(Region::default());

    let get_distribution_request = GetDistributionRequest {
        id: distribution_id,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(get_distribution(&client, get_distribution_request));

    get_distribution_result(resp.clone());

}
