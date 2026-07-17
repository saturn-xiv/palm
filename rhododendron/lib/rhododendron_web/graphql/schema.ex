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

      resolve(&Resolvers.Portal.Locale.by_lang/3)
    end
  end

  mutation do
    @desc "Sign up by email"
    field :sign_up_by_email, type: :succeeded do
      arg(:name, non_null(:string))
      arg(:email, non_null(:string))
      arg(:password, non_null(:string))
      arg(:lang, non_null(:string))
      arg(:timezone, non_null(:string))

      resolve(&Resolvers.Portal.EmailUser.sign_up/3)
    end
  end
end
