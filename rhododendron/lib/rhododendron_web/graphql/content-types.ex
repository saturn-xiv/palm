defmodule RhododendronWeb.Schema.ContentTypes do
  use Absinthe.Schema.Notation
  import_types(Absinthe.Type.Custom)

  @desc "Version"
  object :version do
    field :api, non_null(:string)
    field :iana, non_null(:string)
  end

  @desc "Locale"
  object :locale do
    field :id, non_null(:id)
    field :lang, non_null(:string)
    field :code, non_null(:string)
    field :message, non_null(:string)
    field :inserted_at, non_null(:datetime)
  end

  @desc "Currency"
  object :currency do
    field :id, non_null(:id)
    field :name, non_null(:string)
    field :code, non_null(:string)
    field :country, non_null(:string)
    field :number, non_null(:integer)
    field :units, :integer
  end

  @desc "Succeeded"
  object :succeeded do
    field :created_at, non_null(:datetime)
  end
end
