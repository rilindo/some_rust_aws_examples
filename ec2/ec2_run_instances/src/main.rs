extern crate rusoto_core;
extern crate rusoto_ec2;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    RunInstancesRequest,
    Reservation
};

async fn run_instances(client: &Ec2Client, run_instances_request: RunInstancesRequest) -> Reservation {

    let resp = client.run_instances(run_instances_request).await;
    return resp.unwrap();

}

fn process_run_instances(resp: Reservation) {

    match resp.instances {
        Some(instances) => {
            for i in instances {
                match i.instance_id {
                    Some(instance_id) => println!("{}",instance_id),
                    None => (),
                }
            }
        }
        None => (),
    }
}

fn main() {

    // Run instances. Note that key name is not specified, so you will not be able to
    // login if ssm is not installed and you do not have the proper role assigned to it.

    let matches = App::new("Example Run Instances Using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Run Instances")
                            .arg(Arg::with_name("instance_count")
                                .short("c")
                                .long("instance_count")
                                .help("Set number of instances")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::with_name("security_group_id")
                                .short("g")
                                .long("security_group_id")
                                .help("Set security group id")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::with_name("image_id")
                                .short("i")
                                .long("image_id")
                                .help("Set Image ID")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::with_name("instance_type")
                                .short("t")
                                .long("instance_type")
                                .help("Set instance type")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::with_name("subnet_id")
                                .short("s")
                                .long("subnet_id")
                                .help("Set subnet id")
                                .required(true)
                                .takes_value(true))
                            .get_matches();

    let instance_count = matches.value_of("instance_count").unwrap();
    let security_group_id = matches.value_of("security_group_id").unwrap();
    let image_id = matches.value_of("image_id").unwrap();
    let instance_type = matches.value_of("instance_type").unwrap();
    let subnet_id = matches.value_of("subnet_id").unwrap();

    let client = Ec2Client::new(Region::default());
    let run_instances_request = RunInstancesRequest {
        min_count: instance_count.parse::<i64>().unwrap(),
        max_count: instance_count.parse::<i64>().unwrap(),
        security_group_ids: Some(vec![security_group_id.to_string()]),
        image_id: Some(image_id.to_string()),
        instance_type: Some(instance_type.to_string()),
        subnet_id: Some(subnet_id.to_string()),
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(run_instances(&client, run_instances_request));

    process_run_instances(resp.clone());
}
