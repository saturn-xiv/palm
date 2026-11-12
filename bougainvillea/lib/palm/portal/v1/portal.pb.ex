defmodule Palm.Portal.V1.UserIndexResponse.Item.Type do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.UserIndexResponse.Item.Type",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :EMAIL, 0
  field :SMS, 1
  field :GOOGLE_OAUTH2, 11
  field :WECHAT_OAUTH2, 12
  field :WECHAT_MINI_PROGRAM, 13
end

defmodule Palm.Portal.V1.UserToken.Audience do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.UserToken.Audience",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :SIGN_IN, 0
  field :HYDRANGEA_JOB, 1
end

defmodule Palm.Portal.V1.PasswordRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.PasswordRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :plain, 1, type: :string
  field :salt, 2, type: :double
end

defmodule Palm.Portal.V1.UserIndexResponse.Item do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.UserIndexResponse.Item",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :sn, 1, type: :string
  field :type, 2, type: Palm.Portal.V1.UserIndexResponse.Item.Type, enum: true
end

defmodule Palm.Portal.V1.UserIndexResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.UserIndexResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :items, 1, repeated: true, type: Palm.Portal.V1.UserIndexResponse.Item
end

defmodule Palm.Portal.V1.UserToken.Payload do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.UserToken.Payload",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :type, 1, type: Palm.Portal.V1.UserIndexResponse.Item.Type, enum: true
end

defmodule Palm.Portal.V1.UserToken do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.UserToken",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end
