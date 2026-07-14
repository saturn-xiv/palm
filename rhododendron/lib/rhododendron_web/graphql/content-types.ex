defmodule RhododendronWeb.Schema.ContentTypes do
  use Absinthe.Schema.Notation
  import_types(Absinthe.Type.Custom)

  @desc "Locale"
  object :locale do
    field :id, :id
    field :lang, :string
    field :code, :string
    field :message, :string
    field :inserted_at, :datetime
  end
end
