defmodule Palm.Sms.V1.Task do
  @moduledoc false

  use Protobuf,
    full_name: "palm.sms.v1.Task",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :to, 1, repeated: true, type: :string
  field :body, 2, type: :string
  field :status_callback, 9, proto3_optional: true, type: :string, json_name: "statusCallback"
end
