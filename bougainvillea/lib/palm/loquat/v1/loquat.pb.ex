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

defmodule Palm.Loquat.V1.JwtSignRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.JwtSignRequest",
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

defmodule Palm.Loquat.V1.JwtSignResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.JwtSignResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :token, 1, type: :string
end

defmodule Palm.Loquat.V1.HMacSignRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.HMacSignRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :bytes
end

defmodule Palm.Loquat.V1.HMacSignResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.HMacSignResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :hashed, 1, type: :bytes
end

defmodule Palm.Loquat.V1.HMacVerifyRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.HMacVerifyRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :hashed, 1, type: :bytes
  field :plain, 2, type: :bytes
end

defmodule Palm.Loquat.V1.AesEncryptRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AesEncryptRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :bytes
  field :associated_data, 2, type: :bytes, json_name: "associatedData"
end

defmodule Palm.Loquat.V1.AesEncryptResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AesEncryptResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :cipher, 1, type: :bytes
end

defmodule Palm.Loquat.V1.AesDecryptRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AesDecryptRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :cipher, 1, type: :bytes
  field :associated_data, 2, type: :bytes, json_name: "associatedData"
end

defmodule Palm.Loquat.V1.AesDecryptResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.AesDecryptResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :bytes
end

defmodule Palm.Loquat.V1.Argon2SignRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.Argon2SignRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :password, 1, type: :string
end

defmodule Palm.Loquat.V1.Argon2SignResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.loquat.v1.Argon2SignResponse",
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

defmodule Palm.Loquat.V1.Jwt.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.Jwt", protoc_gen_elixir_version: "0.17.1"

  rpc(:Sign, Palm.Loquat.V1.JwtSignRequest, Palm.Loquat.V1.JwtSignResponse)

  rpc(:Verify, Palm.Loquat.V1.JwtVerifyRequest, Palm.Loquat.V1.JwtVerifyResponse)
end

defmodule Palm.Loquat.V1.Jwt.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.Jwt.Service
end

defmodule Palm.Loquat.V1.HMac.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.HMac", protoc_gen_elixir_version: "0.17.1"

  rpc(:Sign, Palm.Loquat.V1.HMacSignRequest, Palm.Loquat.V1.HMacSignResponse)

  rpc(:Verify, Palm.Loquat.V1.HMacVerifyRequest, Google.Protobuf.Empty)
end

defmodule Palm.Loquat.V1.HMac.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.HMac.Service
end

defmodule Palm.Loquat.V1.Aes.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.Aes", protoc_gen_elixir_version: "0.17.1"

  rpc(:Encrypt, Palm.Loquat.V1.AesEncryptRequest, Palm.Loquat.V1.AesEncryptResponse)

  rpc(:Decrypt, Palm.Loquat.V1.AesDecryptRequest, Palm.Loquat.V1.AesDecryptResponse)
end

defmodule Palm.Loquat.V1.Aes.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.Aes.Service
end

defmodule Palm.Loquat.V1.Argon2.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.loquat.v1.Argon2", protoc_gen_elixir_version: "0.17.1"

  rpc(:Sign, Palm.Loquat.V1.Argon2SignRequest, Palm.Loquat.V1.Argon2SignResponse)

  rpc(:Verify, Palm.Loquat.V1.Argon2VerifyRequest, Google.Protobuf.Empty)
end

defmodule Palm.Loquat.V1.Argon2.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Loquat.V1.Argon2.Service
end
