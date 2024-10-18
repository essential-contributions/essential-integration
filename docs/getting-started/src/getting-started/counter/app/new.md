# Setup Cargo Project

In this section, we'll create a new Cargo project to serve as the front-end application for interacting with the counter contract. Follow the steps below to set up your project and include the necessary dependencies.

## Step 1: Create a New Cargo Project

Run the following command from the root directory of your counter project to create a new Cargo project:

```bash
{{#include ../../../../../code/front-end.sh:cargo-new}}
```

Your project structure should now look like this:

```
counter/
├── contract/
│   ├── pint.toml
│   ├── contract.pnt
│   └── src/
│       └── contract.pnt
└── counter-app/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## Step 2: Add Dependencies

Now, add the necessary dependencies to your Rust project by running the following command:

```bash
{{#include ../../../../../code/front-end.sh:cargo-add}}
```

Your `Cargo.toml` file should now look like this:

```toml
[package]
name = "counter-app"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.89"
clap = { version = "4.5.18", features = ["derive"] }
essential-app-utils = "0.2.0"
essential-deploy-contract = "0.2.0"
essential-hash = "0.3.0"
essential-rest-client = "0.2.0"
essential-types = "0.2.0"
tokio = { version = "1.40.0", features = ["full"] }

[dev-dependencies]
essential-app-utils = { version = "0.2.0", features = ["test-utils"] }
essential-wallet = { version = "0.2.0", features = ["test-utils"] }
```

## Step 3: Add a Test

Lastly, add a test to your front-end application by using the following command:

```bash
{{#include ../../../../../code/front-end.sh:add-test}}
```

After adding the test, your project structure should look like this:

```
counter/
├── contract/
│   ├── pint.toml                        
│   ├── contract.pnt                         
│   └── src/
│       └── contract.pnt                             
└── counter-app/
    ├── Cargo.toml                             
    ├── tests/
    │   └── counter.rs                   
    └── src/
        └── lib.rs    
```

At this point, your Rust project is set up with all the necessary dependencies, and a basic test has been added to your front-end application.
