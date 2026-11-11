defmodule Bougainvillea.Portal.Graphql.ContentTypes do
  use Absinthe.Schema.Notation
  import_types(Absinthe.Type.Custom)

  # ---------------------------------------------------------------------------

  input_object :sign_up_by_email_request do
    field :name, non_null(:string)
    field :email, non_null(:string)
    field :password, non_null(:string)
    field :lang, non_null(:string)
    field :timezone, non_null(:string)
  end

  object :user_sign_in_response do
    field :token, non_null(:string)
    field :lang, non_null(:string)
    field :name, non_null(:string)
    field :avatar, :string
  end

  # ---------------------------------------------------------------------------

  input_object :page do
    field :index, non_null(:integer)
    field :size, non_null(:integer)
  end

  object :pagination do
    field :index, non_null(:integer)
    field :pages, non_null(:integer)
    field :size, non_null(:integer)
    field :total, non_null(:integer)
    field :has_previous, non_null(:boolean)
    field :has_next, non_null(:boolean)
  end

  @desc "Succeed"
  object :succeed do
    field :created_at, non_null(:datetime)
  end

  # ---------------------------------------------------------------------------
end
