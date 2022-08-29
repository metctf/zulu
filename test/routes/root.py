import pytest
from app import create_app


class TestRoutes():

  @pytest.fixture()
  def app(self):
    app = create_app()
    yield app

  @pytest.fixture()
  def client(self, app):
    return app.test_client()

  def test_root_route(self, client):
    response = client.get("/")
    assert(response.status_code == 501)