defmodule RhododendronWeb.Schema.ContentTypes do
  use Absinthe.Schema.Notation
  import_types(Absinthe.Type.Custom)

  @desc "RBAC Resource"
  object :resource do
    field :type, non_null(:string)
    field :id, :integer
  end

  @desc "RBAC Permission"
  object :permission do
    field :object, non_null(:resource)
    field :action, non_null(:string)
  end

  @desc "User layout"
  object :user_layout do
    field :lang, non_null(:string)
    field :timezone, non_null(:string)
    field :name, :string
    field :avatar, :string
    field :roles, non_null(list_of(non_null(:string)))
    field :permissions, non_null(list_of(non_null(:permission)))
  end

  @desc "Site author"
  object :site_author do
    field :name, non_null(:string)
    field :email, non_null(:string)
  end

  @desc "Site layout"
  object :site_layout do
    field :title, non_null(:string)
    field :subhead, non_null(:string)
    field :author, non_null(:site_author)
    field :keywords, non_null(list_of(non_null(:string)))
    field :description, non_null(:string)
    field :copyright, non_null(:string)
    field :available_languages, non_null(list_of(non_null(:string)))
  end

  @desc "Page layout refresh response"
  object :refresh_response do
    field :site, non_null(:site_layout)
    field :user, non_null(:user_layout)
    field :created_at, non_null(:datetime)
  end

  @desc "User sign in response"
  object :user_sign_in_response do
    field :token, non_null(:string)
    field :site, non_null(:site_layout)
    field :user, non_null(:user_layout)
    field :created_at, non_null(:datetime)
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

  @desc "Version"
  object :version do
    field :api, non_null(:string)
    field :iana, non_null(:string)
    field :created_at, non_null(:datetime)
  end

  @desc "Succeeded"
  object :succeeded do
    field :created_at, non_null(:datetime)
  end
end
