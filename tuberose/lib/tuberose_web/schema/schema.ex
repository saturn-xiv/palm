defmodule TuberoseWeb.Schema do
  use Absinthe.Schema
  import_types(TuberoseWeb.Schema.ContentTypes)

  alias TuberoseWeb.Resolvers

  # ---------------------------------------------------------------------------
  query do
    @desc "Get api version"
    field :api_version, non_null(:string) do
      resolve(fn %{}, _ ->
        {:ok, "v#{Application.spec(:tuberose)[:vsn]}"}
      end)
    end

    field :layout, non_null(:layout_response) do
      resolve(&Resolvers.Site.layout/3)
    end

    field :index_locale_by_lang, list_of(non_null(:locale)) do
      arg(:lang, non_null(:string))
      resolve(&Resolvers.Locale.by_lang/3)
    end

    field :show_locale, non_null(:locale) do
      arg(:id, non_null(:id))
      resolve(&Resolvers.Locale.show/3)
    end

    field :index_locale, :index_locale_response do
      arg(:pager, non_null(:pager))
      resolve(&Resolvers.Locale.index/3)
    end

    field :index_leave_word, :index_leave_word_response do
      arg(:pager, non_null(:pager))
      resolve(&Resolvers.LeaveWord.index/3)
    end
  end

  # ---------------------------------------------------------------------------
  mutation do
    field :create_leave_word, non_null(:succeed) do
      arg(:content, non_null(:string))
      arg(:editor, non_null(:text_editor))
      resolve(&Resolvers.LeaveWord.create/3)
    end

    field :user_sign_up_by_email, non_null(:succeed) do
      arg(:request, non_null(:user_sign_up_by_email_request))
      resolve(&Resolvers.EmailUser.sign_up/3)
    end
  end

  # -----------------------------------------------------------------------------
end
