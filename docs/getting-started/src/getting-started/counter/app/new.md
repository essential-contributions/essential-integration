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

**TODO: Update once crates are published**
```toml
[package]
name = "counter-app"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.86"
clap = { version = "4.5.4", features = ["derive"] }
essential-app-utils = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", version = "0.1.0" }
essential-deploy-contract = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", version = "0.1.0" }
essential-hash = { git = "ssh://git@github.com/essential-contributions/essential-base.git", version = "0.1.0" }
essential-rest-client = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", version = "0.1.0" }
essential-types = { git = "ssh://git@github.com/essential-contributions/essential-base.git", version = "0.1.0" }
tokio = { version = "1.38.0", features = ["full"] }

[dev-dependencies]
essential-app-utils = { git = "ssh://git@github.com/essential-contributions/essential-integration.git", features = ["test-utils"] }
essential-wallet = { git = "ssh://git@github.com/essential-contributions/essential-wallet.git", features = ["test-utils"] }
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