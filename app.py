from flask import Flask
from api.routes.root import root_api
from api.routes.accounts import account_api


def create_app()-> Flask:
  """"""
  app = Flask(__name__)
  app.register_blueprint(root_api)
  app.register_blueprint(account_api)
  return app

if __name__ == "__main__":
  app = create_app()
  app.run(host="127.0.0.1", port=5000)