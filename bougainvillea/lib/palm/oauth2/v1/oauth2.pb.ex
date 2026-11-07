defmodule Palm.Oauth2.V1.GoogleSignInUrlRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleSignInUrlRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :redirect_uri, 1, type: :string, json_name: "redirectUri"
  field :scopes, 2, repeated: true, type: :string
end

defmodule Palm.Oauth2.V1.GoogleSignInUrlResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleSignInUrlResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :authorization_url, 1, type: :string, json_name: "authorizationUrl"
  field :state, 2, type: :string
  field :client_id, 3, type: :string, json_name: "clientId"
end

defmodule Palm.Oauth2.V1.GoogleFetchTokenRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleFetchTokenRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :authorization_response, 1, type: :string, json_name: "authorizationResponse"
  field :state, 2, type: :string
  field :scopes, 3, repeated: true, type: :string
end

defmodule Palm.Oauth2.V1.GoogleFetchTokenResponse.Credentials do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleFetchTokenResponse.Credentials",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :token, 1, type: :string
  field :refresh_token, 2, type: :string, json_name: "refreshToken"
  field :granted_scopes, 3, repeated: true, type: :string, json_name: "grantedScopes"
end

defmodule Palm.Oauth2.V1.GoogleFetchTokenResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleFetchTokenResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :credentials, 1, type: Palm.Oauth2.V1.GoogleFetchTokenResponse.Credentials
  field :client_id, 2, type: :string, json_name: "clientId"
end

defmodule Palm.Oauth2.V1.GoogleRevokeResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleRevokeResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.Oauth2.V1.GoogleUserInfoResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.oauth2.v1.GoogleUserInfoResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :sub, 1, type: :string
  field :email, 2, type: :string
  field :email_verified, 3, type: :bool, json_name: "emailVerified"
  field :name, 4, proto3_optional: true, type: :string
  field :given_name, 5, proto3_optional: true, type: :string, json_name: "givenName"
  field :family_name, 6, proto3_optional: true, type: :string, json_name: "familyName"
  field :picture, 7, proto3_optional: true, type: :string
  field :locale, 8, proto3_optional: true, type: :string
  field :hosted_domain, 9, proto3_optional: true, type: :string, json_name: "hostedDomain"
  field :gender, 10, proto3_optional: true, type: :string
  field :link, 11, proto3_optional: true, type: :string
end

defmodule Palm.Oauth2.V1.GoogleService.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.oauth2.v1.GoogleService", protoc_gen_elixir_version: "0.17.1"

  rpc(:SignInUrl, Palm.Oauth2.V1.GoogleSignInUrlRequest, Palm.Oauth2.V1.GoogleSignInUrlResponse)

  rpc(
    :FetchToken,
    Palm.Oauth2.V1.GoogleFetchTokenRequest,
    Palm.Oauth2.V1.GoogleFetchTokenResponse
  )

  rpc(
    :UserInfo,
    Palm.Oauth2.V1.GoogleFetchTokenResponse.Credentials,
    Palm.Oauth2.V1.GoogleUserInfoResponse
  )

  rpc(
    :Revoke,
    Palm.Oauth2.V1.GoogleFetchTokenResponse.Credentials,
    Palm.Oauth2.V1.GoogleRevokeResponse
  )
end

defmodule Palm.Oauth2.V1.GoogleService.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Oauth2.V1.GoogleService.Service
end
