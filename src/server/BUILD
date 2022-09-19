py_binary(
  name="server",
  srcs=["server.py"],
  deps=["//src/protobufs:zulu_grpc"],
)

py_test(
  name="server_test",
  srcs=["server_test.py"],
  deps=[
    "//src/protobufs:zulu_grpc",
    ":server",
    ]
)