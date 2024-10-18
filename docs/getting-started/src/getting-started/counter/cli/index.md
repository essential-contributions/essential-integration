# Rust CLI

After deploying the counter application, you’ll need a way to interact with it. While you could use `curl` to manually create and send solutions, this approach can be inefficient and error-prone.

Instead, we will create a simple Command Line Interface (CLI) in Rust to interact with the counter. This CLI will reuse much of the functionality you've already written in the app and will streamline interactions with the deployed application.