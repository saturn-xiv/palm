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

    field :index_route, list_of(:route) do
      resolve(&Resolvers.Site.routes/3)
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

    field :confirm_user_by_email, non_null(:succeed) do
      arg(:user, non_null(:string))
      arg(:home, non_null(:string))
      resolve(&Resolvers.EmailUser.confirm/3)
    end

    field :unlock_user_by_email, non_null(:succeed) do
      arg(:user, non_null(:string))
      arg(:home, non_null(:string))
      resolve(&Resolvers.EmailUser.unlock/3)
    end

    field :forgot_user_password_by_email, non_null(:succeed) do
      arg(:user, non_null(:string))
      arg(:home, non_null(:string))
      resolve(&Resolvers.EmailUser.forgot_password/3)
    end

    field :current_email_user, non_null(:current_email_user) do
      resolve(&Resolvers.EmailUser.current/3)
    end

    field :current_user, non_null(:current_user) do
      resolve(&Resolvers.User.current/3)
    end

    field :index_log, non_null(:index_log_response) do
      arg(:pager, non_null(:pager))
      resolve(&Resolvers.User.logs/3)
    end

    field :index_leave_word, :index_leave_word_response do
      arg(:pager, non_null(:pager))
      resolve(&Resolvers.LeaveWord.index/3)
    end

    field :show_attachment, :attachment do
      arg(:id, non_null(:id))
      resolve(&Resolvers.Attachment.show/3)
    end

    field :index_attachment, :index_attachment_response do
      arg(:pager, non_null(:pager))
      resolve(&Resolvers.Attachment.index/3)
    end

    field :index_attachment_by_images, list_of(non_null(:attachment)) do
      resolve(&Resolvers.Attachment.by_image/3)
    end
  end

  # ---------------------------------------------------------------------------
  mutation do
    field :sign_in_user_by_email, non_null(:user_sign_in_response) do
      arg(:user, non_null(:string))
      arg(:password, non_null(:string))
      arg(:ttl, non_null(:integer))
      resolve(&Resolvers.User.sign_in_by_email/3)
    end

    field :sign_up_user_by_email, non_null(:succeed) do
      arg(:real_name, non_null(:string))
      arg(:nickname, non_null(:string))
      arg(:email, non_null(:string))
      arg(:password, non_null(:string))
      arg(:home, non_null(:string))
      arg(:timezone, non_null(:string))
      resolve(&Resolvers.EmailUser.sign_up/3)
    end

    field :confirm_user_by_email_token, non_null(:succeed) do
      arg(:token, non_null(:string))
      resolve(&Resolvers.EmailUser.confirm_by_token/3)
    end

    field :unlock_user_by_email_token, non_null(:succeed) do
      arg(:token, non_null(:string))
      resolve(&Resolvers.EmailUser.unlock_by_token/3)
    end

    field :reset_user_password_by_email, non_null(:succeed) do
      arg(:token, non_null(:string))
      arg(:password, non_null(:string))
      arg(:home, non_null(:string))
      resolve(&Resolvers.EmailUser.reset_password/3)
    end

    field :change_user_password_by_email, non_null(:succeed) do
      arg(:home, non_null(:string))
      arg(:current_password, non_null(:string))
      arg(:new_password, non_null(:string))
      resolve(&Resolvers.EmailUser.change_password/3)
    end

    field :sign_out_user, non_null(:succeed) do
      resolve(&Resolvers.User.sign_out/3)
    end

    field :update_user_profile, non_null(:succeed) do
      arg(:real_name, non_null(:string))
      arg(:avatar, non_null(:string))
      arg(:lang, non_null(:string))
      arg(:timezone, non_null(:string))
      resolve(&Resolvers.User.update_profile/3)
    end

    field :cancel_account, non_null(:succeed) do
      arg(:home, non_null(:string))
      arg(:reason, :string)
      resolve(&Resolvers.User.cancel/3)
    end

    field :create_leave_word, non_null(:succeed) do
      arg(:content, non_null(:string))
      arg(:editor, non_null(:text_editor))
      resolve(&Resolvers.LeaveWord.create/3)
    end

    field :set_attachment_uploaded, non_null(:succeed) do
      arg(:bucket, non_null(:string))
      arg(:object, non_null(:string))
      resolve(&Resolvers.Attachment.set_uploaded/3)
    end

    field :upload_attachment_url, non_null(:upload_attachment_url_response) do
      arg(:title, non_null(:string))
      arg(:content_type, non_null(:string))
      arg(:size, non_null(:integer))
      arg(:ttl, non_null(:integer))
      resolve(&Resolvers.Attachment.upload_url/3)
    end

    field :update_attachment, non_null(:succeed) do
      arg(:id, non_null(:id))
      arg(:title, non_null(:string))
      resolve(&Resolvers.Attachment.update/3)
    end

    field :destroy_attachment, non_null(:succeed) do
      arg(:id, non_null(:id))
      resolve(&Resolvers.Attachment.destroy/3)
    end
  end

  # -----------------------------------------------------------------------------
end
