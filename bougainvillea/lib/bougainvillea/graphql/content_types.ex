defmodule Bougainvillea.Graphql.ContentTypes do
  use Absinthe.Schema.Notation
  import_types(Absinthe.Type.Custom)

  @desc "Succeed"
  object :succeed do
    field :created_at, :datetime
  end
end
