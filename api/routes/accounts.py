"""
Defines the routes for account related operations.
"""
from flask import Blueprint, abort

account_api = Blueprint("accounts", __name__)

@account_api.post("/login")
def login() -> str:
  abort(501);