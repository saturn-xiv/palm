defmodule Palm.Cups.V1.Task.JobSheet do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.cups.v1.Task.JobSheet",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :STANDARD, 0
  field :UNCLASSIFIED, 1
  field :CLASSIFIED, 2
  field :CONFIDENTIAL, 3
  field :SECRET, 4
  field :TOP_SECRET, 5
end

defmodule Palm.Cups.V1.Task.Media do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.cups.v1.Task.Media",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :A4, 0
  field :A3, 2
  field :LETTER, 3
end

defmodule Palm.Cups.V1.Task.Orientation do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.cups.v1.Task.Orientation",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :LABDSCAPE_COUNTER_CLOCKWISE90, 0
  field :LABDSCAPE_CLOCKWISE90, 1
  field :REVERSE_PORTRAIT, 2
end

defmodule Palm.Cups.V1.Task.Quality do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.cups.v1.Task.Quality",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :DRAFT, 0
  field :NORMAL, 1
  field :BEST, 2
end

defmodule Palm.Cups.V1.Task.Sides do
  @moduledoc false

  use Protobuf,
    enum: true,
    full_name: "palm.cups.v1.Task.Sides",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :ONE, 0
  field :TWO_LONG, 1
  field :TWO_SHORT, 2
end

defmodule Palm.Cups.V1.Task do
  @moduledoc false

  use Protobuf,
    full_name: "palm.cups.v1.Task",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3

  field :name, 1, type: :string
  field :document, 2, type: :bytes
  field :number_up, 8, type: :uint32, json_name: "numberUp"
  field :copies, 9, type: :uint32
  field :job_sheet, 11, type: Palm.Cups.V1.Task.JobSheet, json_name: "jobSheet", enum: true
  field :media, 12, type: Palm.Cups.V1.Task.Media, enum: true
  field :orientation, 13, type: Palm.Cups.V1.Task.Orientation, enum: true
  field :quality, 14, type: Palm.Cups.V1.Task.Quality, enum: true
  field :sides, 15, type: Palm.Cups.V1.Task.Sides, enum: true
end
