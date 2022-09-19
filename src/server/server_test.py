from concurrent import futures
import unittest
import grpc
from src.protobufs.zulu_pb2 import Account, GetAccountRequest 
from src.protobufs.zulu_pb2_grpc import ZuluStub, add_ZuluServicer_to_server
from server import ZuluService

class TestAccountServer(unittest.TestCase):
  def setUp(self):
    port = "127.0.0.1:50051"
    self._server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    add_ZuluServicer_to_server(ZuluService(), self._server)
    self._server.add_insecure_port(port)
    self._server.start()

    channel = grpc.insecure_channel(port)
    self._stub = ZuluStub(channel)
  
  def tearDown(self):
    self._server.stop(None)
  
  def assert_proto_equal(self, expected, actual):
    for field in expected.DESCRIPTOR.fields:
      expected_value = getattr(expected, field.name)
      actual_value = getattr(actual, field.name)
      self.assertEqual(
        expected_value,
        actual_value,
        f'{expected.DESCRIPTOR.name}.{field.name} expected {expected_value}: got {actual_value}'
      )

  def test_GetAccount_withValidId_returns_account(self):
    dummy_account = Account(user_id=12, student_id="12", full_name="Sunny Donaldson")
    request = GetAccountRequest(user_id=12)
    response = self._stub.GetAccount(request)
    self.assert_proto_equal(dummy_account, response.account)
    self.assertEqual(response.account.user_id, dummy_account.user_id)
  
  def test_GetAccount_withInvalidId_aborts(self):
    request = GetAccountRequest()
    with self.assertRaises(grpc.RpcError):
      response = self._stub.GetAccount(request)
      self.assertFalse(response)

if __name__ == '__main__':
    unittest.main()