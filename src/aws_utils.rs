use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest, ListServicesRequest,
                 ListTasksRequest, StopTaskRequest, UpdateServiceRequest, UpdateServiceResponse};

pub async fn fetch_cluster_names(client: &EcsClient) -> Result<Vec<String>, anyhow::Error> {
    let request = ListClustersRequest::default();
    let output = client.list_clusters(request).await?;
    let cluster_arns = output.cluster_arns.unwrap_or_else(Vec::new);
    Ok(cluster_arns)
}

pub async fn fetch_service_arns(client: &EcsClient, cluster_arn: &str) -> Result<Vec<String>, anyhow::Error> {
    let request = ListServicesRequest {
        cluster: Some(cluster_arn.to_string()),
        ..Default::default()
    };
    let output = client.list_services(request).await?;
    let service_arns = output.service_arns.unwrap_or_else(Vec::new);
    Ok(service_arns)
}

pub async fn revive_ecs_service(client: &EcsClient, cluster_arn: &str, service_arn: &str) -> Result<(), anyhow::Error> {
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

pub async fn pause_ecs_service(client: &EcsClient, cluster_arn: &str, service_arn: &str) -> Result<(), anyhow::Error> {

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