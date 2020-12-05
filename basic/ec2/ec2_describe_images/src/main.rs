extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeImagesRequest, Ec2, Ec2Client, Filter};

#[tokio::main]
async fn main() {
    let client = Ec2Client::new(Region::default());

    let filter = Filter {
        name: Some(String::from("is-public")),
        values: Some(vec![String::from("false")]),
    };

    let describe_images_request = DescribeImagesRequest {
        filters: Some(vec![filter]),
        ..Default::default()
    };

    match client.describe_images(describe_images_request).await {
        Ok(output) => match output.images {
            Some(images) => {
                for i in images {
                    match i.name {
                        Some(name) => println!("Name: {}, image id: {}", name, i.image_id.unwrap()),
                        None => println!("Name: Unknown, image id: {}", i.image_id.unwrap()),
                    }
                }
            }
            None => println!("No images returned in this region"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
