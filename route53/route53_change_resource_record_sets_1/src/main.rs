extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_core::Region;
use rusoto_route53::{Route53,
    Route53Client,
    ResourceRecordSet,
    ResourceRecord,
    ChangeResourceRecordSetsRequest,
    Change,
    ChangeBatch
};

#[tokio::main]
async fn main() {
    let client = Route53Client::new(Region::UsEast1);
    let my_name = "mydomain.example.com".to_string();
    let my_hosted_zone_id = "MYRECORDSETID";
    let my_ttl: i64 = "300".parse().expect("Not a number!");
    let my_ip = "192.168.19.16".to_string();
    let my_resource_record = ResourceRecord {
        value: my_ip.clone()
    };

    let my_resource_records_vec = vec![my_resource_record];

    let my_type = "A".to_string();
    let create_record_set_req = ResourceRecordSet {
        name: my_name.clone(),
        type_: my_type.clone(),
        ttl: Some(my_ttl.clone()),
        resource_records: Some(my_resource_records_vec.clone()),
        ..Default::default()
    };

    let my_change_vec = Change {
        action: "UPSERT".to_string(),
        resource_record_set: create_record_set_req.clone()
    };

    let my_changes_vec = vec![my_change_vec];

    let my_change_batch_vec = ChangeBatch {
        changes: my_changes_vec.clone(),
        ..Default::default()
    };

    let change_resource_record_sets_req = ChangeResourceRecordSetsRequest {
        change_batch: my_change_batch_vec.clone(),
        hosted_zone_id: my_hosted_zone_id.to_string()
    };

    let resp = client.change_resource_record_sets(change_resource_record_sets_req).await;

    println!(
        "RecordSet {} created, resp: {:#?}",
        my_name.clone(),
        resp.unwrap()
    );

}
