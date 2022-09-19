from concurrent import futures
import grpc
from src.protobufs.zulu_pb2 import Account, GetAccountRequest, GetAccountResponse
from src.protobufs.zulu_pb2_grpc import ZuluServicer, add_ZuluServicer_to_server

dummy_account = Account(user_id=12, student_id="12", full_name="Sunny Donaldson")

class ZuluService (ZuluServicer):
  def GetAccount(self, request, context) -> GetAccountResponse:
    if request.user_id == 0:
      context.set_code(grpc.StatusCode.NOT_FOUND)
      context.set_message(f"No account with id: {request.user_id}")

    return GetAccountResponse(account=dummy_account)
    
def serve(port="127.0.0.1:50051"):
  server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
  add_ZuluServicer_to_server(ZuluService(), server)
  server.add_insecure_port(port)
  server.start()
  server.wait_for_termination()

if __name__ == "__main__":
  serve()