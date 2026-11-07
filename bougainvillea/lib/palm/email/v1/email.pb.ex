defmodule Palm.Email.V1.Task.Address do
  @moduledoc false

  use Protobuf,
    full_name: "palm.email.v1.Task.Address",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :name, 1, proto3_optional: true, type: :string
  field :email, 2, type: :string
end

defmodule Palm.Email.V1.Task.Body do
  @moduledoc false

  use Protobuf,
    full_name: "palm.email.v1.Task.Body",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :html, 1, type: :bool
  field :content, 2, type: :string
end

defmodule Palm.Email.V1.Task.Attachment do
  @moduledoc false

  use Protobuf,
    full_name: "palm.email.v1.Task.Attachment",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :inline_id, 1, proto3_optional: true, type: :string, json_name: "inlineId"
  field :name, 2, type: :string
  field :content_type, 3, type: :string, json_name: "contentType"
  field :content, 9, type: :bytes
end

defmodule Palm.Email.V1.Task do
  @moduledoc false

  use Protobuf,
    full_name: "palm.email.v1.Task",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :to, 1, type: Palm.Email.V1.Task.Address

  field :reply_to, 2,
    proto3_optional: true,
    type: Palm.Email.V1.Task.Address,
    json_name: "replyTo"

  field :cc, 3, repeated: true, type: Palm.Email.V1.Task.Address
  field :bcc, 4, repeated: true, type: Palm.Email.V1.Task.Address
  field :subject, 11, type: :string
  field :body, 12, type: Palm.Email.V1.Task.Body
  field :attachments, 19, repeated: true, type: Palm.Email.V1.Task.Attachment
end
