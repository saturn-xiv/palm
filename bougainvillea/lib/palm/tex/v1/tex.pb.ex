defmodule Palm.Tex.V1.Task.Class do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.tex.v1.Task.Class",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :ARTICLE, 0
  field :REPORT, 1
  field :BOOK, 2
  field :LETTER, 3
  field :BEAMER, 4
end

defmodule Palm.Tex.V1.Task.FilesEntry do
  @moduledoc false

  use Protobuf,
    full_name: "palm.tex.v1.Task.FilesEntry",
    map: true,
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :key, 1, type: :string
  field :value, 2, type: :bytes
end

defmodule Palm.Tex.V1.Task do
  @moduledoc false

  use Protobuf,
    full_name: "palm.tex.v1.Task",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :entry, 1, type: :string
  field :class, 2, type: Palm.Tex.V1.Task.Class, enum: true
  field :files, 3, repeated: true, type: Palm.Tex.V1.Task.FilesEntry, map: true
  field :output, 4, type: :string
end
