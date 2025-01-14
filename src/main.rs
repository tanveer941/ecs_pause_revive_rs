mod aws_utils;

use rusoto_core::Region;
use rusoto_ecs::{Ecs, EcsClient};
use tokio;
use inquire::{Select, InquireError};
use crate::aws_utils::{fetch_cluster_names, fetch_service_arns, pause_ecs_service, revive_ecs_service};

fn main() {
    println!("Starting the ECS client...");
    let client = EcsClient::new(Region::UsEast1);

    let cluster_arns = tokio::runtime::Runtime::new().unwrap().block_on(fetch_cluster_names(&client));
    let cluster_arns = cluster_arns.unwrap();
    let cluster_name_choice: Result<&str, InquireError> = Select::new("Choose your cluster?", cluster_arns.iter().map(|s| s.as_str()).collect()).prompt();
    match cluster_name_choice {
        Ok(cluster_name_chosen) => println!("The chosen cluster is {}", cluster_name_chosen),
        Err(_) => println!("There was an error choosing the cluster name"),
    }
    let ch1 = cluster_name_choice.unwrap();
    println!("Listing the related services of the cluster...");
    let service_arns = tokio::runtime::Runtime::new().unwrap().block_on(fetch_service_arns(&client, ch1));
    let service_arns = service_arns.unwrap();
    let service_name_choice: Result<&str, InquireError> = Select::new("Choose your service from the cluster?", service_arns.iter().map(|s| s.as_str()).collect()).prompt();
    match service_name_choice {
        Ok(service_name_chosen) => println!("The chosen service is {}", service_name_chosen),
        Err(_) => println!("There was an error choosing the service name from cluster"),
    }

    let options: Vec<&str> = vec!["pause", "revive"];
    let pause_revive_choice: Result<&str, InquireError> = Select::new("Choose whether to pause or revive the ECS service?", options).prompt();

    match pause_revive_choice {
        Ok(pause_revive_chosen) => println!("{} the service", pause_revive_chosen),
        Err(_) => println!("There was an error, please try again"),
    }

    if pause_revive_choice.unwrap() == "pause" {
        println!("Pausing the service...");
        tokio::runtime::Runtime::new().unwrap().block_on(pause_ecs_service(&client, ch1, service_name_choice.unwrap()));
    } else {
        println!("Reviving the service...");
        tokio::runtime::Runtime::new().unwrap().block_on(revive_ecs_service(&client, ch1, service_name_choice.unwrap()));
    }
}
