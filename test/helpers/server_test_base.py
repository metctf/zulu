from app import create_app
import pytest

class ServerTestBase():
  @pytest.fixture()
  def app(self):
    app = create_app()
    yield app

  @pytest.fixture()
  def client(self, app):
    return app.test_client()