defmodule RhododendronWeb.Schema do
  use Absinthe.Schema

  import_types(RhododendronWeb.Schema.ContentTypes)
  alias RhododendronWeb.Resolvers

  query do
    @desc "Get locales by language"
    field :locale_by_lang, list_of(:locale) do
      arg(:lang, non_null(:string))

      resolve(&Resolvers.Locale.by_lang/3)
    end
  end
end
