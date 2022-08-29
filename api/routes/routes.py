""" 
Contains all the routes for the server.
"""
from flask import abort, Blueprint

routes = Blueprint("/", __name__)
@routes.route("/")
def root():
  abort(501)
