# Essential Rest Client
This is a rust library and cli tool that allows you to easily make rest requests to a `essential-rest-server`.

```
Essential REST Client

Usage: essential-rest-client [ADDRESS] [COMMAND]

Commands:
  deploy-contract           Deploy a contract to the server
  check-solution            Check a solution against the server
  check-solution-with-data  Check a solution against the server with data
  submit-solution           Submit a solution to the server
  solution-outcome          Get the outcome of a solution
  get-predicate             Get a predicate from a contract
  get-contract              Get a contract from the server
  list-contracts            List contracts on the server
  list-solutions-pool       List solutions in the pool on the server
  list-winning-blocks       List winning blocks on the server
  query-state               Query the state of a contract
  help                      Print this message or the help of the given subcommand(s)

Arguments:
  [ADDRESS]  Server address to bind to [default: http://0.0.0.0:0]

Options:
  -h, --help     Print help
  -V, --version  Print version
  ```