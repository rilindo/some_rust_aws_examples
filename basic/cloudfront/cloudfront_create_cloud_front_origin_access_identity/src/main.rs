extern crate rusoto_cloudfront;
extern crate rusoto_core;

use std::time::{SystemTime, UNIX_EPOCH};

use rusoto_cloudfront::{
    CloudFront, CloudFrontClient, CloudFrontOriginAccessIdentityConfig,
    CreateCloudFrontOriginAccessIdentityRequest, CreateCloudFrontOriginAccessIdentityResult,
};
use rusoto_core::Region;

async fn create_cloud_front_origin_access_identity(
    client: &CloudFrontClient,
    create_cloud_front_origin_access_identity_request: CreateCloudFrontOriginAccessIdentityRequest,
) -> CreateCloudFrontOriginAccessIdentityResult {
    let resp = client
        .create_cloud_front_origin_access_identity(
            create_cloud_front_origin_access_identity_request,
        )
        .await;
    return resp.unwrap();
}

fn create_cloud_front_origin_access_identity_result(
    resp: CreateCloudFrontOriginAccessIdentityResult,
) {
    match resp.cloud_front_origin_access_identity {
        Some(cloud_front_origin_access_identity) => {
            println!(
                "CloudFront Origin Identity ID: {}",
                cloud_front_origin_access_identity.id
            )
        }
        None => (),
    }
}

fn main() {
    let client = CloudFrontClient::new(Region::default());

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let caller_reference = since_the_epoch.as_millis();
    let cloudfront_config = CloudFrontOriginAccessIdentityConfig {
        caller_reference: caller_reference.to_string(),
        comment: "This is a CloudFront Identity Config".to_string(),
    };

    let create_cloud_front_origin_access_identity_request =
        CreateCloudFrontOriginAccessIdentityRequest {
            cloud_front_origin_access_identity_config: cloudfront_config,
        };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(create_cloud_front_origin_access_identity(
        &client,
        create_cloud_front_origin_access_identity_request,
    ));

    create_cloud_front_origin_access_identity_result(resp.clone());
}
