defmodule Palm.Sms.V1.Task.By do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.sms.v1.Task.By",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :TWILIO, 0
end

defmodule Palm.Sms.V1.Task do
  @moduledoc false

  use Protobuf,
    full_name: "palm.sms.v1.Task",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :from, 1, type: :string
  field :to, 2, repeated: true, type: :string
  field :body, 3, type: :string
  field :status_callback, 8, proto3_optional: true, type: :string, json_name: "statusCallback"
  field :by, 9, type: Palm.Sms.V1.Task.By, enum: true
end
