from flask import Flask
from api.routes.routes import routes

def create_app()-> Flask:
  """"""
  app = Flask(__name__)
  app.register_blueprint(routes)
  return app

if __name__ == "__main__":
  app = create_app()
  app.run(host="127.0.0.1", port=5000)