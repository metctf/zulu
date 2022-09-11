# Stuff for Developers.
## First Time set-up.
1. Install virtualenv if you haven't already: `pip install virtualenv`
1. Clone the repository, and navigate to the root of it.
1. Create a virtual environment for the project: `python3 venv zuluapi-env`
1. Activate the virtual environment: `. zuluapi-env/bin/activate`
1. Install the dependencies `pip -r requirements.txt`
1. Create user `zulu` in mysql `sudo mysql -e "CREATE USER zulu@localhost IDENTIFIED BY 'password'"`
1. Create the database `sudo mysql < zulu.sql`

## Running the Server.
1. Make sure you've done the setup above.
1. Navigate to the root of project
1. Run `bazel run //src/accounts:server`

## Running the Tests.
Nothing here yet :)

## Useful Resources
[bazel python rules]("https://bazel.build/reference/be/python")

[gRPC docs]("https://grpc.io/docs/languages/python/quickstart/")

[protobuf docs]("https://developers.google.com/protocol-buffers/docs/overview")

[rules-proto-grpc for python]("https://rules-proto-grpc.com/en/latest/lang/python.html")

