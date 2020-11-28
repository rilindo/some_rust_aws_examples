extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_core::Region;
use rusoto_route53::{
    Change, ChangeBatch, ChangeResourceRecordSetsRequest, ResourceRecord, ResourceRecordSet,
    Route53, Route53Client,
};

use std::env;
use std::error::Error;
use std::process;

struct NameServerRecord {
    record_name: String,
    record_type: String,
    ip: String,
    ttl: i64,
    hosted_zone_id: String,
}

impl NameServerRecord {
    pub fn new(args: &[String]) -> Result<NameServerRecord, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }

        let record_name = args[1].clone();
        let record_type = args[2].clone();
        let ip = args[3].clone();
        let ttl: i64 = args[4].parse::<i64>().unwrap();
        let hosted_zone_id = args[5].clone();

        Ok(NameServerRecord {
            record_name,
            record_type,
            ip,
            ttl,
            hosted_zone_id,
        })
    }
}

fn create_name_record(config: NameServerRecord) -> Result<(), Box<dyn Error>> {
    let record_name = config.record_name;
    let record_type = config.record_type;
    let hosted_zone_id = config.hosted_zone_id;
    let ttl = config.ttl;
    let ip = config.ip;

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(route53_request(
        &record_name,
        &record_type,
        &hosted_zone_id,
        ttl,
        &ip,
    ));
    Ok(())
}

async fn route53_request(
    record_name: &str,
    record_type: &str,
    hosted_zone_id: &str,
    ttl: i64,
    ip: &str,
) {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = Route53Client::new(Region::default());

    let resource_record = ResourceRecord {
        value: ip.to_string(),
    };

    let resource_records = vec![resource_record];

    let create_record_set_req = ResourceRecordSet {
        name: record_name.to_string(),
        type_: record_type.to_string(),
        ttl: Some(ttl),
        resource_records: Some(resource_records.clone()),
        ..Default::default()
    };

    let change = Change {
        action: "UPSERT".to_string(),
        resource_record_set: create_record_set_req.clone(),
    };

    let changes = vec![change];

    let change_batch = ChangeBatch {
        changes: changes.clone(),
        ..Default::default()
    };

    let change_resource_record_sets_request = ChangeResourceRecordSetsRequest {
        change_batch: change_batch.clone(),
        hosted_zone_id: hosted_zone_id.to_string(),
    };

    let resp = client
        .change_resource_record_sets(change_resource_record_sets_request)
        .await;

    println!(
        "RecordSet {} created, resp: {:#?}",
        record_name.to_string(),
        resp.unwrap()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let name_server_record = NameServerRecord::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = create_name_record(name_server_record) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
