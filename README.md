# ecs_pause_revive_rs
A command line utility to pause and revive ECS containers when not in use to save cost


## Blueprint of the Application
- The application will be a command line utility
- Choose the cluster
- Choose the service
- Choose the option to pause or revive

### commands to add the dependent packages

```
cargo add tokio --features full
cargo add rusoto_core
cargo add rusoto_ecs
cargo add inquire
cargo add anyhow
```

### To run the application
```
cargo run
target/debug/ecs_pause_revive_rs
```


