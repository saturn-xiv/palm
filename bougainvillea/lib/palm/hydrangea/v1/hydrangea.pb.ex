defmodule Palm.Hydrangea.V1.StashJobRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.hydrangea.v1.StashJobRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :sn, 1, type: :string
  field :status_code, 2, type: :int32, json_name: "statusCode"
  field :stdout, 3, type: :string
  field :stderr, 4, type: :string
  field :launched_at, 8, type: Google.Protobuf.Timestamp, json_name: "launchedAt"
  field :elapsed, 9, type: Google.Protobuf.Duration
end

defmodule Palm.Hydrangea.V1.Job do
  @moduledoc false

  use Protobuf,
    full_name: "palm.hydrangea.v1.Job",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :sn, 1, type: :string
  field :summary, 2, type: :string
  field :version, 3, type: :string
  field :working_dir, 11, type: :string, json_name: "workingDir"
  field :script, 12, type: :string
  field :args, 13, repeated: true, type: :string
end

defmodule Palm.Hydrangea.V1.Stash.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.hydrangea.v1.Stash", protoc_gen_elixir_version: "0.17.1"

  rpc(:Job, Palm.Hydrangea.V1.StashJobRequest, Google.Protobuf.Empty)
end

defmodule Palm.Hydrangea.V1.Stash.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Hydrangea.V1.Stash.Service
end
