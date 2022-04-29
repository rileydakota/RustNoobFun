use rusoto_core::{Region};
use rusoto_iam::{IamClient, Iam, ListRolesRequest, ListRolesResponse};
use tokio;

#[tokio::main]
async fn main() {
    let client = IamClient::new(Region::UsEast1);
    let list_roles_input = Default::default();
    let results = client.list_roles(list_roles_input).await.unwrap();
    
    match results.is_truncated {
        Some(x) => println!("Response Truncated: {}", x),
        None => println!("Response not truncated")
    };

    for role in results.roles {
        println!("{}", role.arn)
    }
    }

