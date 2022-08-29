from test.helpers.server_test_base import ServerTestBase


class TestAccounts(ServerTestBase):
  def test_login_returns_unimplemented(self, client):
    response = client.post("/accounts/login")
    assert(response.status_code == 501)
