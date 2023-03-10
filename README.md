# Zulu
## What is Zulu?
Zulu is a CTF server currently in development by the Cardiff Met CTF society
for use with our challenges produced by members of our society. It will allow
us to keep track of the number of flags and points associated with them for
each member of our society, which will be displayed on a society-wide
leaderboard. Zulu will be designed to be university agnostic and highly
configurable, allowing other university societies to use our software for their
own uses if they so wish, with their own branding and such. Zulu is licensed
under the GPLv3 and is free software.

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
1. Run `bazel run //src/server`

## Running the Tests.
1. Run `bazel test --test_output=all //src/server:server_test`

The `--test_output=all` flag is optional, but it saves having to open the testlog file,
as all the output gets displayed in the console.
Nothing here yet :)

## Useful Resources.
[bazel python rules](https://bazel.build/reference/be/python)

[gRPC docs](https://grpc.io/docs/languages/python/quickstart/)

[protobuf docs](https://developers.google.com/protocol-buffers/docs/overview)

[rules-proto-grpc for python](https://rules-proto-grpc.com/en/latest/lang/python.html)

