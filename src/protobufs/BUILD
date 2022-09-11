load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_proto_grpc//python:defs.bzl", "python_grpc_library")

proto_library(
  name = "accounts_proto",
  srcs = ["accounts.proto"],
)

python_grpc_library(
    name = "accounts_grpc",
    protos = [":accounts_proto"],
    visibility=["//visibility:public"]
)
