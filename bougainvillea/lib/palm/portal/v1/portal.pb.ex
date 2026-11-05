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

defmodule Palm.Portal.V1.TexTask.Class do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.TexTask.Class",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :Article, 0
  field :Report, 1
  field :Book, 2
  field :Letter, 3
  field :Beamer, 4
end

defmodule Palm.Portal.V1.CupsTask.JobSheet do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.CupsTask.JobSheet",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :Standard, 0
  field :Unclassified, 1
  field :Classified, 2
  field :Confidential, 3
  field :Secret, 4
  field :TopSecret, 5
end

defmodule Palm.Portal.V1.CupsTask.Media do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.CupsTask.Media",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :A4, 0
  field :A3, 2
  field :Letter, 3
end

defmodule Palm.Portal.V1.CupsTask.Orientation do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.CupsTask.Orientation",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :LandscapeCounterClockwise90, 0
  field :LandscapeClockwise90, 1
  field :ReversePortrait, 2
end

defmodule Palm.Portal.V1.CupsTask.Quality do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.CupsTask.Quality",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :Draft, 0
  field :Normal, 1
  field :Best, 2
end

defmodule Palm.Portal.V1.CupsTask.Sides do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.portal.v1.CupsTask.Sides",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :One, 0
  field :TwoLong, 1
  field :TwoShort, 2
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

defmodule Palm.Portal.V1.SmsTask do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.SmsTask",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :to, 1, repeated: true, type: :string
  field :body, 2, type: :string
  field :status_callback, 9, proto3_optional: true, type: :string, json_name: "statusCallback"
end

defmodule Palm.Portal.V1.TexTask.FilesEntry do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.TexTask.FilesEntry",
    map: true,
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :key, 1, type: :string
  field :value, 2, type: :bytes
end

defmodule Palm.Portal.V1.TexTask do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.TexTask",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :entry, 1, type: :string
  field :class, 2, type: Palm.Portal.V1.TexTask.Class, enum: true
  field :files, 3, repeated: true, type: Palm.Portal.V1.TexTask.FilesEntry, map: true
  field :output, 4, type: :string
end

defmodule Palm.Portal.V1.EmailTask.Address do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.EmailTask.Address",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :name, 1, proto3_optional: true, type: :string
  field :email, 2, type: :string
end

defmodule Palm.Portal.V1.EmailTask.Body do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.EmailTask.Body",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :html, 1, type: :bool
  field :content, 2, type: :string
end

defmodule Palm.Portal.V1.EmailTask.Attachment do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.EmailTask.Attachment",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :inline_id, 1, proto3_optional: true, type: :string, json_name: "inlineId"
  field :name, 2, type: :string
  field :content_type, 3, type: :string, json_name: "contentType"
  field :content, 9, type: :bytes
end

defmodule Palm.Portal.V1.EmailTask do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.EmailTask",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :to, 1, type: Palm.Portal.V1.EmailTask.Address

  field :reply_to, 2,
    proto3_optional: true,
    type: Palm.Portal.V1.EmailTask.Address,
    json_name: "replyTo"

  field :cc, 3, repeated: true, type: Palm.Portal.V1.EmailTask.Address
  field :bcc, 4, repeated: true, type: Palm.Portal.V1.EmailTask.Address
  field :subject, 11, type: :string
  field :body, 12, type: Palm.Portal.V1.EmailTask.Body
  field :attachments, 19, repeated: true, type: Palm.Portal.V1.EmailTask.Attachment
end

defmodule Palm.Portal.V1.CupsTask do
  @moduledoc false

  use Protobuf,
    full_name: "palm.portal.v1.CupsTask",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :name, 1, type: :string
  field :document, 2, type: :bytes
  field :number_up, 8, type: :uint32, json_name: "numberUp"
  field :copies, 9, type: :uint32
  field :job_sheet, 11, type: Palm.Portal.V1.CupsTask.JobSheet, json_name: "jobSheet", enum: true
  field :media, 12, type: Palm.Portal.V1.CupsTask.Media, enum: true
  field :orientation, 13, type: Palm.Portal.V1.CupsTask.Orientation, enum: true
  field :quality, 14, type: Palm.Portal.V1.CupsTask.Quality, enum: true
  field :sides, 15, type: Palm.Portal.V1.CupsTask.Sides, enum: true
end
