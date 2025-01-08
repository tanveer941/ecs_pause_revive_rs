use rusoto_core::Region;
use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest, ListServicesRequest};
use rusoto_ecs::{UpdateServiceRequest, UpdateServiceResponse, ListTasksRequest, StopTaskRequest};
use tokio;
use inquire::{Select, InquireError};


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


async fn fetch_cluster_names(client: &EcsClient) -> Result<Vec<String>, anyhow::Error> {
    let request = ListClustersRequest::default();
    let output = client.list_clusters(request).await?;
    let cluster_arns = output.cluster_arns.unwrap_or_else(Vec::new);
    Ok(cluster_arns)
}

async fn fetch_service_arns(client: &EcsClient, cluster_arn: &str) -> Result<Vec<String>, anyhow::Error> {
    let request = ListServicesRequest {
        cluster: Some(cluster_arn.to_string()),
        ..Default::default()
    };
    let output = client.list_services(request).await?;
    let service_arns = output.service_arns.unwrap_or_else(Vec::new);
    Ok(service_arns)
}

async fn revive_ecs_service(client: &EcsClient, cluster_arn: &str, service_arn: &str) -> Result<(), anyhow::Error> {
    println!("Reviving the ECS service...");
    let request = UpdateServiceRequest {
        cluster: Some(cluster_arn.to_string()),
        service: service_arn.to_string(),
        desired_count: Some(2), // Set the desired count to greater than 1
        ..Default::default()
    };

    let response: UpdateServiceResponse = client.update_service(request).await?;
    println!("Service updated: {:?}", response);
    Ok(())
}

async fn pause_ecs_service(client: &EcsClient, cluster_arn: &str, service_arn: &str) -> Result<(), anyhow::Error> {

    println!("Pausing the ECS service...");
    let request = UpdateServiceRequest {
        cluster: Some(cluster_arn.to_string()),
        service: service_arn.to_string(),
        desired_count: Some(0), // Set the desired count to greater than 1
        ..Default::default()
    };

    let response: UpdateServiceResponse = client.update_service(request).await?;
    println!("Service updated: {:?}", response);
    // List the tasks for the service to see if they are stopped
    let tasks = client.list_tasks(ListTasksRequest {
        cluster: Some(cluster_arn.to_string()),
        service_name: Some(service_arn.to_string()),
        ..Default::default()
    }).await?;
    // iterate through the tasks and stop them
    for task_arn in tasks.task_arns.unwrap_or_else(Vec::new) {
        let _ = client.stop_task(StopTaskRequest {
            cluster: Some(cluster_arn.to_string()),
            task: task_arn,
            ..Default::default()
        }).await?;
    }

    Ok(())
}