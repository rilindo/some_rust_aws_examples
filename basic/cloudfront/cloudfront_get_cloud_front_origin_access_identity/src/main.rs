extern crate rusoto_cloudfront;
extern crate rusoto_core;

extern crate clap;
use clap::{App, Arg};

use rusoto_cloudfront::{
    CloudFront, CloudFrontClient, GetCloudFrontOriginAccessIdentityRequest, GetCloudFrontOriginAccessIdentityResult,
};
use rusoto_core::Region;

async fn get_cloud_front_origin_access_identity(
    client: &CloudFrontClient,
    get_cloud_front_origin_access_identity: GetCloudFrontOriginAccessIdentityRequest,
) -> GetCloudFrontOriginAccessIdentityResult {
    let resp = client.get_cloud_front_origin_access_identity(get_cloud_front_origin_access_identity).await;
    return resp.unwrap();
}

fn get_cloud_front_origin_access_identity_result(resp: GetCloudFrontOriginAccessIdentityResult) {
    match resp.cloud_front_origin_access_identity {
        Some(cloud_front_origin_access_identity) => {
            match cloud_front_origin_access_identity.cloud_front_origin_access_identity_config {
                Some(cloud_front_origin_access_identity_config) => {
                    println!("Caller ID: {}, Caller Reference: {}", cloud_front_origin_access_identity.id, cloud_front_origin_access_identity_config.caller_reference)
                }
                None => ()
            }
        }
        None => ()
    }
}

fn main() {
    let matches = App::new("Example of a get cloudfront identity call using Rust and Rusoto" )
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Get cloudfront origin access id")
        .arg(
            Arg::with_name("origin_id")
                .short("i")
                .long("origin_id")
                .help("Set origin ID")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let origin_id = matches.value_of("origin_id").unwrap().to_string();

    let client = CloudFrontClient::new(Region::default());

    let get_cloud_front_origin_access_identity_req = GetCloudFrontOriginAccessIdentityRequest {
        id: origin_id,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(get_cloud_front_origin_access_identity(&client, get_cloud_front_origin_access_identity_req));

    get_cloud_front_origin_access_identity_result(resp.clone());
}
