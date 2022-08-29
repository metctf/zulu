# Stuff for Developers.
## First Time set-up.
1. Install virtualenv if you haven't already: `pip install virtualenv`
1. Clone the repository, and navigate to the root of it.
1. Create a virtual environment for the project: `python3 venv zuluapi-env`
1. Activate the virtual environment: `. zuluapi-env/bin/activate`
1. Install the dependencies `pip -r requirements.txt`

## Running the Server.
1. Make sure you've done the setup above.
1. Navigate to the root of project
1. Run `flask --debug run`

The --debug flag (among other things) automatically reloads the server when changes are detected.

## Running the Tests.
Just run `pytest` to execute all the tests.

## Useful Resources
* [Flask API Best Practices](https://auth0.com/blog/best-practices-for-flask-api-development/)
