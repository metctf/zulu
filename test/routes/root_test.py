from test.helpers.server_test_base import ServerTestBase

class TestRoutes(ServerTestBase):
  def test_root_route(self, client):
    response = client.get("/")
    assert(response.status_code == 501)