import grpc
# from protobufs import Account, GetAccountRequest, GetAccountResponse
from src.protobufs.accounts_pb2 import Account, GetAccountRequest, GetAccountResponse
from src.protobufs.accounts_pb2_grpc import AccountsServicer, add_AccountsServicer_to_server
from concurrent import futures

dummy_account = Account(user_id=12, student_id="12", full_name="Sunny Donaldson")

class AccountsService (AccountsServicer):
  def GetAccount(self, request, context) -> GetAccountResponse:
    if request.user_id == None:
      context.abort(grpc.StatusCode.NOT_FOUND, "Account not found")
    return GetAccountResponse(account=dummy_account)
    
def serve():
  server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
  add_AccountsServicer_to_server(AccountsService(), server)
  server.add_insecure_port("127.0.0.1:50051")
  server.start()
  print("server running")
  server.wait_for_termination()

if __name__ == "__main__":
  serve()