# A package around the `apps/counter/test.sh` script.
#
# This package automatically starts an instance of the `essential-rest-server`,
# runs the test script, then closes the server.
{ essential-minimal
, essential-cli
, jq
, xxd
, writeShellApplication
}:
let
  src = ./../apps/counter;
in
writeShellApplication {
  name = "test-app-counter";
  runtimeInputs = [ essential-minimal essential-cli jq xxd ];
  text = ''
    # Function to clean up and kill the server.
    server_pid=""
    cleanup() {
      echo "Shutting down the server with PID $server_pid..."
      kill $server_pid
    }
    trap cleanup EXIT HUP INT QUIT TERM

    # Default port number, or receive via arg.
    SERVER_PORT="45539"
    echo $
    if [ $# -gt 0 ]; then
      SERVER_PORT=$1
      echo "Using port $SERVER_PORT"
    else
      echo "No port specified, using $SERVER_PORT"
    fi

    # Run the server in the background.
    RUST_LOG=trace essential-rest-server --db memory "0.0.0.0:$SERVER_PORT" &
    server_pid=$!

    # Give the server a second to startup.
    sleep 1

    # Run the app test command.
    ${src}/test.sh "$SERVER_PORT"
  '';
}
