"""
Defines the routes for account related operations.
"""
from flask import Blueprint, abort

account_api = Blueprint("/accounts", __name__)

@account_api.route("/login")
def login():
  abort(501);