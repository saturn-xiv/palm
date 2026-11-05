defmodule Bougainvillea.Graphql.Schema do
  use Absinthe.Schema
  import_types(Bougainvillea.Graphql.ContentTypes)

  alias Bougainvillea.Graphql.Resolvers

  query do
    @desc "Show version"
    field :version, :string do
      resolve(&Resolvers.Portal.version/3)
    end
  end

  mutation do
    @desc "Sign up by email"
    field :sign_up_by_email, type: :succeed do
      arg(:name, non_null(:string))
      arg(:email, non_null(:string))
      arg(:password, non_null(:string))
      arg(:lang, non_null(:string))
      arg(:timezone, non_null(:string))

      resolve(&Resolvers.Portal.EmailUser.sign_up/3)
    end
  end
end
