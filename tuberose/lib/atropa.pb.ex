defmodule Palm.Atropa.V1.AesPlainMessage do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :payload, 1, type: :bytes
end

defmodule Palm.Atropa.V1.AesCodeMessage do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :payload, 1, type: :bytes
  field :salt, 2, type: :bytes
end

defmodule Palm.Atropa.V1.JwtSignRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :key_id, 1, proto3_optional: true, type: :string, json_name: "keyId"
  field :issuer, 11, type: :string
  field :subject, 12, type: :string
  field :audiences, 13, repeated: true, type: :string
  field :not_before, 18, type: Google.Protobuf.Timestamp, json_name: "notBefore"
  field :expires_at, 19, type: Google.Protobuf.Timestamp, json_name: "expiresAt"
  field :extra, 99, proto3_optional: true, type: :string
end

defmodule Palm.Atropa.V1.JwtSignResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :token, 1, type: :string
end

defmodule Palm.Atropa.V1.JwtVerifyRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :token, 1, type: :string
  field :issuer, 2, type: :string
  field :audience, 3, type: :string
end

defmodule Palm.Atropa.V1.JwtVerifyResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :jwt_id, 1, type: :string, json_name: "jwtId"
  field :key_id, 2, proto3_optional: true, type: :string, json_name: "keyId"
  field :subject, 11, type: :string
  field :extra, 19, proto3_optional: true, type: :string
end

defmodule Palm.Atropa.V1.HMacSignRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :plain, 1, type: :bytes
end

defmodule Palm.Atropa.V1.HMacSignResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :code, 1, type: :bytes
end

defmodule Palm.Atropa.V1.HMacVerifyRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :plain, 1, type: :bytes
  field :code, 2, type: :bytes
end

defmodule Palm.Atropa.V1.PolicyHasRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :user, 1, type: Palm.Atropa.V1.PolicyUsersResponse.Item
  field :role, 2, type: Palm.Atropa.V1.PolicyRolesResponse.Item
end

defmodule Palm.Atropa.V1.PolicyCanRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :user, 1, type: Palm.Atropa.V1.PolicyUsersResponse.Item
  field :resource, 2, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Resource
  field :operation, 3, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation
end

defmodule Palm.Atropa.V1.PolicyUsersResponse.Item do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  oneof :by, 0

  field :i, 2, type: :int64, oneof: 0
  field :s, 9, type: :string, oneof: 0
end

defmodule Palm.Atropa.V1.PolicyUsersResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :items, 1, repeated: true, type: Palm.Atropa.V1.PolicyUsersResponse.Item
end

defmodule Palm.Atropa.V1.PolicyRolesResponse.Item.Administrator do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyRolesResponse.Item.Root do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyRolesResponse.Item do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  oneof :by, 0

  field :root, 1, type: Palm.Atropa.V1.PolicyRolesResponse.Item.Root, oneof: 0
  field :administrator, 2, type: Palm.Atropa.V1.PolicyRolesResponse.Item.Administrator, oneof: 0
  field :code, 9, type: :string, oneof: 0
end

defmodule Palm.Atropa.V1.PolicyRolesResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :items, 1, repeated: true, type: Palm.Atropa.V1.PolicyRolesResponse.Item
end

defmodule Palm.Atropa.V1.PolicyRolesForUserRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :user, 1, type: Palm.Atropa.V1.PolicyUsersResponse.Item
  field :roles, 2, repeated: true, type: Palm.Atropa.V1.PolicyRolesResponse.Item
end

defmodule Palm.Atropa.V1.PolicyPermissionsForUserRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :user, 1, type: Palm.Atropa.V1.PolicyUsersResponse.Item
  field :permissions, 2, repeated: true, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item
end

defmodule Palm.Atropa.V1.PolicyPermissionsForRoleRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :role, 1, type: Palm.Atropa.V1.PolicyRolesResponse.Item
  field :permissions, 2, repeated: true, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Resource do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :type, 1, type: :string
  field :id, 2, proto3_optional: true, type: :int64
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Read do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Write do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Append do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Execute do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Credit do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Debit do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Inquiry do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  oneof :by, 0

  field :read, 1, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Read, oneof: 0
  field :write, 2, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Write, oneof: 0
  field :append, 3, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Append, oneof: 0

  field :execute, 4,
    type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Execute,
    oneof: 0

  field :credit, 5, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Credit, oneof: 0
  field :debit, 6, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Debit, oneof: 0

  field :inquiry, 7,
    type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation.Inquiry,
    oneof: 0

  field :code, 99, type: :string, oneof: 0
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse.Item do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :resource, 1, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Resource
  field :operation, 2, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item.Operation
end

defmodule Palm.Atropa.V1.PolicyPermissionsResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :items, 1, repeated: true, type: Palm.Atropa.V1.PolicyPermissionsResponse.Item
end

defmodule Palm.Atropa.V1.S3CreateBucketRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :name, 1, type: :string
  field :public, 2, type: :bool
  field :expiration_days, 9, type: :int32, json_name: "expirationDays"
end

defmodule Palm.Atropa.V1.S3CreateBucketResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :name, 1, type: :string
end

defmodule Palm.Atropa.V1.S3UploadRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :bucket, 1, type: :string
  field :object, 2, type: :string
  field :ttl, 9, type: Google.Protobuf.Duration
end

defmodule Palm.Atropa.V1.S3UrlResponse do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :url, 1, type: :string
end

defmodule Palm.Atropa.V1.S3PresignedUrlRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :bucket, 1, type: :string
  field :object, 2, type: :string
  field :title, 3, type: :string
  field :ttl, 9, type: Google.Protobuf.Duration
end

defmodule Palm.Atropa.V1.S3PermanentUrlRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :bucket, 1, type: :string
  field :object, 2, type: :string
end

defmodule Palm.Atropa.V1.SmsSendRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :to, 1, repeated: true, type: :string
  field :body, 2, type: :string
  field :callback_uri, 3, proto3_optional: true, type: :string, json_name: "callbackUri"
end

defmodule Palm.Atropa.V1.EmailSendRequest.Address do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :name, 1, type: :string
  field :email, 2, type: :string
end

defmodule Palm.Atropa.V1.EmailSendRequest.Body do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :text, 1, type: :string
  field :html, 2, type: :bool
end

defmodule Palm.Atropa.V1.EmailSendRequest.Attachment do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :title, 1, type: :string
  field :content_type, 2, type: :string, json_name: "contentType"
  field :inline, 3, type: :bool
  field :body, 9, type: :bytes
end

defmodule Palm.Atropa.V1.EmailSendRequest do
  @moduledoc false

  use Protobuf, syntax: :proto3, protoc_gen_elixir_version: "0.12.0"

  field :subject, 1, type: :string
  field :body, 2, type: Palm.Atropa.V1.EmailSendRequest.Body
  field :to, 3, type: Palm.Atropa.V1.EmailSendRequest.Address
  field :cc, 4, repeated: true, type: Palm.Atropa.V1.EmailSendRequest.Address
  field :bcc, 5, repeated: true, type: Palm.Atropa.V1.EmailSendRequest.Address
  field :attachments, 9, repeated: true, type: Palm.Atropa.V1.EmailSendRequest.Attachment
end

defmodule Palm.Atropa.V1.Aes.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.atropa.v1.Aes", protoc_gen_elixir_version: "0.12.0"

  rpc :Encrypt, Palm.Atropa.V1.AesPlainMessage, Palm.Atropa.V1.AesCodeMessage

  rpc :Decrypt, Palm.Atropa.V1.AesCodeMessage, Palm.Atropa.V1.AesPlainMessage
end

defmodule Palm.Atropa.V1.Aes.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Atropa.V1.Aes.Service
end

defmodule Palm.Atropa.V1.Jwt.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.atropa.v1.Jwt", protoc_gen_elixir_version: "0.12.0"

  rpc :Sign, Palm.Atropa.V1.JwtSignRequest, Palm.Atropa.V1.JwtSignResponse

  rpc :Verify, Palm.Atropa.V1.JwtVerifyRequest, Palm.Atropa.V1.JwtVerifyResponse
end

defmodule Palm.Atropa.V1.Jwt.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Atropa.V1.Jwt.Service
end

defmodule Palm.Atropa.V1.HMac.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.atropa.v1.HMac", protoc_gen_elixir_version: "0.12.0"

  rpc :Sign, Palm.Atropa.V1.HMacSignRequest, Palm.Atropa.V1.HMacSignResponse

  rpc :Verify, Palm.Atropa.V1.HMacVerifyRequest, Google.Protobuf.Empty
end

defmodule Palm.Atropa.V1.HMac.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Atropa.V1.HMac.Service
end

defmodule Palm.Atropa.V1.Policy.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.atropa.v1.Policy", protoc_gen_elixir_version: "0.12.0"

  rpc :Has, Palm.Atropa.V1.PolicyHasRequest, Google.Protobuf.Empty

  rpc :Can, Palm.Atropa.V1.PolicyCanRequest, Google.Protobuf.Empty

  rpc :DeleteUser, Palm.Atropa.V1.PolicyUsersResponse.Item, Google.Protobuf.Empty

  rpc :DeleteRole, Palm.Atropa.V1.PolicyRolesResponse.Item, Google.Protobuf.Empty

  rpc :GetRolesForUser,
      Palm.Atropa.V1.PolicyUsersResponse.Item,
      Palm.Atropa.V1.PolicyRolesResponse

  rpc :GetImplicitRolesForUser,
      Palm.Atropa.V1.PolicyUsersResponse.Item,
      Palm.Atropa.V1.PolicyRolesResponse

  rpc :GetUsersForRole,
      Palm.Atropa.V1.PolicyRolesResponse.Item,
      Palm.Atropa.V1.PolicyUsersResponse

  rpc :GetImplicitUsersForRole,
      Palm.Atropa.V1.PolicyRolesResponse.Item,
      Palm.Atropa.V1.PolicyUsersResponse

  rpc :AddRolesForUser, Palm.Atropa.V1.PolicyRolesForUserRequest, Google.Protobuf.Empty

  rpc :DeleteRolesForUser, Palm.Atropa.V1.PolicyRolesForUserRequest, Google.Protobuf.Empty

  rpc :GetPermissionsForUser,
      Palm.Atropa.V1.PolicyUsersResponse.Item,
      Palm.Atropa.V1.PolicyPermissionsResponse

  rpc :GetImplicitPermissionsForUser,
      Palm.Atropa.V1.PolicyUsersResponse.Item,
      Palm.Atropa.V1.PolicyPermissionsResponse

  rpc :AddPermissionsForUser,
      Palm.Atropa.V1.PolicyPermissionsForUserRequest,
      Google.Protobuf.Empty

  rpc :DeletePermissionsForUser,
      Palm.Atropa.V1.PolicyPermissionsForUserRequest,
      Google.Protobuf.Empty

  rpc :GetPermissionsForRole,
      Palm.Atropa.V1.PolicyRolesResponse.Item,
      Palm.Atropa.V1.PolicyPermissionsResponse

  rpc :GetImplicitPermissionsForRole,
      Palm.Atropa.V1.PolicyRolesResponse.Item,
      Palm.Atropa.V1.PolicyPermissionsResponse

  rpc :AddPermissionsForRole,
      Palm.Atropa.V1.PolicyPermissionsForRoleRequest,
      Google.Protobuf.Empty

  rpc :DeletePermissionsForRole,
      Palm.Atropa.V1.PolicyPermissionsForRoleRequest,
      Google.Protobuf.Empty
end

defmodule Palm.Atropa.V1.Policy.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Atropa.V1.Policy.Service
end

defmodule Palm.Atropa.V1.S3.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.atropa.v1.S3", protoc_gen_elixir_version: "0.12.0"

  rpc :CreateBucket, Palm.Atropa.V1.S3CreateBucketRequest, Palm.Atropa.V1.S3CreateBucketResponse

  rpc :Upload, Palm.Atropa.V1.S3UploadRequest, Palm.Atropa.V1.S3UrlResponse

  rpc :PermanentUrl, Palm.Atropa.V1.S3PermanentUrlRequest, Palm.Atropa.V1.S3UrlResponse

  rpc :PresignedUrl, Palm.Atropa.V1.S3PresignedUrlRequest, Palm.Atropa.V1.S3UrlResponse
end

defmodule Palm.Atropa.V1.S3.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.Atropa.V1.S3.Service
end