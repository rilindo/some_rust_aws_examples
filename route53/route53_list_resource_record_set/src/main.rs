extern crate clap;
extern crate rusoto_core;
extern crate rusoto_route53;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_route53::{
    ListResourceRecordSetsRequest, ListResourceRecordSetsResponse, Route53, Route53Client,
};

async fn list_resource_record_sets(
    client: &Route53Client,
    list_resource_record_sets_request: ListResourceRecordSetsRequest,
) -> ListResourceRecordSetsResponse {
    let resp = client
        .list_resource_record_sets(list_resource_record_sets_request)
        .await;
    return resp.unwrap();
}

fn process_list_resource_record_sets(resp: ListResourceRecordSetsResponse) {
    for r in resp.resource_record_sets {
        match r.resource_records {
            Some(resource_records) => {
                for rr in resource_records {
                    println!("Record Name: {}, Record Values: {}", r.name, rr.value)
                }
            }
            None => (),
        }
        match r.alias_target {
            Some(alias_target) => println!(
                "Record name: {}, DNS Target: {}",
                r.name, alias_target.dns_name
            ),
            None => (),
        }
    }
}

fn main() {
    let matches = App::new("Example List Record Set Using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("List Record Sets")
        .arg(
            Arg::with_name("HOSTZONEID")
                .help("Host Zone ID")
                .required(true)
                .index(1),
        )
        .get_matches();
    let host_zone_id = matches.value_of("HOSTZONEID").unwrap();

    let client = Route53Client::new(Region::default());
    let list_resource_record_sets_request = ListResourceRecordSetsRequest {
        hosted_zone_id: host_zone_id.to_string(),
        start_record_identifier: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let mut resp = rt.block_on(list_resource_record_sets(
        &client,
        list_resource_record_sets_request,
    ));

    process_list_resource_record_sets(resp.clone());

    while resp.clone().is_truncated {
        let list_resource_record_sets_request = ListResourceRecordSetsRequest {
            hosted_zone_id: host_zone_id.to_string(),
            start_record_identifier: resp.clone().next_record_identifier,
            ..Default::default()
        };
        resp = rt.block_on(list_resource_record_sets(
            &client,
            list_resource_record_sets_request,
        ));
        process_list_resource_record_sets(resp.clone());
    }
}
