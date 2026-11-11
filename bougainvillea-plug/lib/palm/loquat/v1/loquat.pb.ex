defmodule Palm.Loquat.V1.JwtVerifyRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.JwtVerifyRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :token, 1, type: :string
  field :issuer, 2, type: :string
  field :audience, 3, type: :string
end

defmodule Palm.Loquat.V1.JwtVerifyResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.JwtVerifyResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :subject, 1, type: :string
  field :payload, 2, proto3_optional: true, type: :bytes
end

defmodule Palm.Loquat.V1.JwtComputeRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.JwtComputeRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :jwt_id, 1, proto3_optional: true, type: :string, json_name: "jwtId"
  field :key_id, 2, proto3_optional: true, type: :string, json_name: "keyId"
  field :issuer, 3, type: :string
  field :subject, 4, type: :string
  field :audiences, 5, repeated: true, type: :string
  field :issued_at, 6, type: :int64, json_name: "issuedAt"
  field :not_before, 7, type: :int64, json_name: "notBefore"
  field :expired_at, 8, type: :int64, json_name: "expiredAt"
  field :payload, 9, proto3_optional: true, type: :bytes
end

defmodule Palm.Loquat.V1.JwtComputeResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.JwtComputeResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :token, 1, type: :string
end

defmodule Palm.Loquat.V1.MacComputeRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.MacComputeRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :bytes
end

defmodule Palm.Loquat.V1.MacComputeResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.MacComputeResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :hashed, 1, type: :bytes
end

defmodule Palm.Loquat.V1.MacVerifyRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.MacVerifyRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :hashed, 1, type: :bytes
  field :plain, 2, type: :bytes
end

defmodule Palm.Loquat.V1.AeadEncryptRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AeadEncryptRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :bytes
  field :associated_data, 2, type: :bytes, json_name: "associatedData"
end

defmodule Palm.Loquat.V1.AeadEncryptResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AeadEncryptResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :cipher, 1, type: :bytes
end

defmodule Palm.Loquat.V1.AeadDecryptRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AeadDecryptRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :cipher, 1, type: :bytes
  field :associated_data, 2, type: :bytes, json_name: "associatedData"
end

defmodule Palm.Loquat.V1.AeadDecryptResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AeadDecryptResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :bytes
end

defmodule Palm.Loquat.V1.Argon2ComputeRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.Argon2ComputeRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :password, 1, type: :string
end

defmodule Palm.Loquat.V1.Argon2ComputeResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.Argon2ComputeResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :hashed, 1, type: :string
end

defmodule Palm.Loquat.V1.Argon2VerifyRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.Argon2VerifyRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :hashed, 1, type: :string
  field :password, 2, type: :string
end

defmodule Palm.Loquat.V1.JwtService.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.JwtService", protoc_gen_elixir_version: "0.17.1"

  rpc(:Compute, Palm.Loquat.V1.JwtComputeRequest, Palm.Loquat.V1.JwtComputeResponse)

  rpc(:Verify, Palm.Loquat.V1.JwtVerifyRequest, Palm.Loquat.V1.JwtVerifyResponse)
end

defmodule Palm.Loquat.V1.JwtService.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.JwtService.Service
end

defmodule Palm.Loquat.V1.MacService.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.MacService", protoc_gen_elixir_version: "0.17.1"

  rpc(:Compute, Palm.Loquat.V1.MacComputeRequest, Palm.Loquat.V1.MacComputeResponse)

  rpc(:Verify, Palm.Loquat.V1.MacVerifyRequest, Google.Protobuf.Empty)
end

defmodule Palm.Loquat.V1.MacService.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.MacService.Service
end

defmodule Palm.Loquat.V1.AeadService.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.AeadService", protoc_gen_elixir_version: "0.17.1"

  rpc(:Encrypt, Palm.Loquat.V1.AeadEncryptRequest, Palm.Loquat.V1.AeadEncryptResponse)

  rpc(:Decrypt, Palm.Loquat.V1.AeadDecryptRequest, Palm.Loquat.V1.AeadDecryptResponse)
end

defmodule Palm.Loquat.V1.AeadService.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.AeadService.Service
end

defmodule Palm.Loquat.V1.Argon2Service.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.Argon2Service", protoc_gen_elixir_version: "0.17.1"

  rpc(:Compute, Palm.Loquat.V1.Argon2ComputeRequest, Palm.Loquat.V1.Argon2ComputeResponse)

  rpc(:Verify, Palm.Loquat.V1.Argon2VerifyRequest, Google.Protobuf.Empty)
end

defmodule Palm.Loquat.V1.Argon2Service.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.Argon2Service.Service
end
