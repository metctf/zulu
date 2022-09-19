load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_proto_grpc//python:defs.bzl", "python_grpc_library")

proto_library(
  name = "zulu_proto",
  srcs = ["zulu.proto"],
)

python_grpc_library(
    name = "zulu_grpc",
    protos = [":zulu_proto"],
    visibility=["//visibility:public"],
)
