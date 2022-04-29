use aws_config;
use aws_sdk_iam;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iam::Client::new(&config);
    //let list_roles_input = Default::default();
    //let results = client.list_roles(list_roles_input).await.unwrap();
    let results = client.list_roles().send().await.unwrap();
    let roles = results.roles.unwrap();

    for role in roles {
        match role.role_name {
            Some(x) => println!("{}", x),
            None => panic!()
        }
    }
    }

