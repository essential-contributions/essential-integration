# Create the cargo project

Create a new cargo project by running following command from the root of your counter project:

```bash
{{#include ../../../../../code/front-end.sh:cargo-new}}
```
You project should look like:
```
counter/contract/pint.toml
counter/contract/contract.pnt
counter/contract/src/contract.pnt
counter/counter-app/Cargo.toml
counter/counter-app/src/lib.rs
```
## Dependencies
Now add the following dependencies to your rust project:
```bash
{{#include ../../../../../code/front-end.sh:cargo-add}}
```
Your cargo toml should look something like this:

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

## Add a test
Lastly add a test to your front end application:
```bash
{{#include ../../../../../code/front-end.sh:add-test}}
```
Your project should now look like:
```
counter/contract/pint.toml                        
counter/contract/contract.pnt                         
counter/contract/src/contract.pnt                             
counter/counter-app/Cargo.toml                             
counter/counter-app/tests/counter.rs                   
counter/counter-app/src/lib.rs    
```
