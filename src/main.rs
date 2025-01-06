use rusoto_core::Region;
use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest};
use tokio;
use inquire::{Select, InquireError};


fn main() {
    println!("Starting the ECS client...");
    let cluster_arns = tokio::runtime::Runtime::new().unwrap().block_on(fetchClusterNames());
    let cluster_arns = cluster_arns.unwrap();
    let cluster_name_choice: Result<&str, InquireError> = Select::new("Choose your cluster?", cluster_arns.iter().map(|s| s.as_str()).collect()).prompt();
    match cluster_name_choice {
        Ok(cluster_name_chosen) => println!("The chosen cluster is {}", cluster_name_chosen),
        Err(_) => println!("There was an error choosing the cluster name"),
    }
}


async fn fetchClusterNames() -> Result<Vec<String>, rusoto_core::RusotoError<rusoto_ecs::ListClustersError>> {
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