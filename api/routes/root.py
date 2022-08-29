""" 
Contains the root routes for the server.
"""
from flask import abort, Blueprint

root_api = Blueprint("/", __name__)
@root_api.route("/")
def home()-> str :
  abort(501)
