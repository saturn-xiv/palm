defmodule RhododendronWeb.Schema do
  use Absinthe.Schema

  import_types(RhododendronWeb.Schema.ContentTypes)
  alias RhododendronWeb.Resolvers

  query do
    @desc "API version"
    field :version, non_null(:version) do
      resolve(&Resolvers.Portal.version/3)
    end

    @desc "Time zones"
    field :timezones, list_of(non_null(:string)) do
      resolve(&Resolvers.Portal.timezones/3)
    end

    @desc "Currencies"
    field :currencies, list_of(non_null(:currency)) do
      resolve(&Resolvers.Portal.currencies/3)
    end

    @desc "Get locales by language"
    field :locale_by_lang, list_of(non_null(:locale)) do
      arg(:lang, non_null(:string))

      resolve(&Resolvers.Locale.by_lang/3)
    end
  end
end
