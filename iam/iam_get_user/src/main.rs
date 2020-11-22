extern crate rusoto_core;
extern crate rusoto_iam;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    GetUserRequest,
};

#[tokio::main]
async fn main() {

    let matches = App::new("Example Get User Using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Describe Security Groups")
                            .arg(Arg::with_name("USERNAME")
                               .help("USERNAME")
                               .required(true)
                               .index(1))
                              .get_matches();
    let user_name = matches.value_of("USERNAME").unwrap();

    let client = IamClient::new(Region::default());
    let get_user_request = GetUserRequest {
        user_name: Some(user_name.clone().to_string()),
        ..Default::default()
    };

    match client.get_user(get_user_request).await {
        Ok(output) => {
            println!("User Name: {}", output.user.user_name);
            println!("User ARN: {}", output.user.arn);
            println!("Create Date: {}", output.user.create_date);
            println!("User ID: {}", output.user.user_id);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
