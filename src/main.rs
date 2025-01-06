use rusoto_core::Region;
use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest, ListServicesRequest, ListServicesError};
use tokio;
use inquire::{Select, InquireError};


fn main() {
    println!("Starting the ECS client...");
    let cluster_arns = tokio::runtime::Runtime::new().unwrap().block_on(fetch_cluster_names());
    let cluster_arns = cluster_arns.unwrap();
    let cluster_name_choice: Result<&str, InquireError> = Select::new("Choose your cluster?", cluster_arns.iter().map(|s| s.as_str()).collect()).prompt();
    match cluster_name_choice {
        Ok(cluster_name_chosen) => println!("The chosen cluster is {}", cluster_name_chosen),
        Err(_) => println!("There was an error choosing the cluster name"),
    }
    println!("Listing the related services of the cluster...");
    let service_arns = tokio::runtime::Runtime::new().unwrap().block_on(fetch_service_arns(&cluster_name_choice.unwrap()));
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
    } else {
        println!("Reviving the service...");
    }
}


async fn fetch_cluster_names() -> Result<Vec<String>, rusoto_core::RusotoError<rusoto_ecs::ListClustersError>> {
    let client = EcsClient::new(Region::UsEast1);

    let request = ListClustersRequest::default();

    match client.list_clusters(request).await {
        Ok(output) => {
            if let Some(cluster_arns) = output.cluster_arns {
                Ok(cluster_arns)
            } else {
                Ok(vec![])
            }
        }
        Err(error) => Err(error),
    }
}


async fn fetch_service_arns(cluster_arn: &str) -> Result<Vec<String>, rusoto_core::RusotoError<ListServicesError>> {
    let client = EcsClient::new(Region::UsEast1);

    let request = ListServicesRequest {
        cluster: Some(cluster_arn.to_string()),
        ..Default::default()
    };

    match client.list_services(request).await {
        Ok(output) => {
            if let Some(service_arns) = output.service_arns {
                Ok(service_arns)
            } else {
                Ok(vec![])
            }
        }
        Err(error) => Err(error),
    }
}