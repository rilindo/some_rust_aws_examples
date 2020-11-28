extern crate rusoto_core;
extern crate rusoto_ec2;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    TerminateInstancesRequest,
    TerminateInstancesResult
};

async fn terminate_instances(client: &Ec2Client, terminate_instances_request: TerminateInstancesRequest) -> TerminateInstancesResult {

    let resp = client.terminate_instances(terminate_instances_request).await;
    return resp.unwrap();

}

fn process_terminate_instances(resp: TerminateInstancesResult) {

    match resp.terminating_instances {
        Some(terminating_instances) => {
            for i in terminating_instances {
                match i.instance_id {
                    Some(instance_id) => println!("Terminating {}",instance_id),
                    None => (),
                }
            }
        }
        None => (),
    }
}

fn main() {

    let matches = App::new("Example Terminate Instances Using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Terminate Instances")
                            .arg(Arg::with_name("instance_id")
                                .short("i")
                                .long("instance_id")
                                .help("Set instance_id")
                                .required(true)
                                .takes_value(true))

                            .get_matches();

    let instance_id = matches.value_of("instance_id").unwrap();


    let client = Ec2Client::new(Region::default());
    let terminate_instances_request = TerminateInstancesRequest {
        instance_ids: vec![instance_id.to_string()],
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(terminate_instances(&client, terminate_instances_request));

    process_terminate_instances(resp.clone());
}
