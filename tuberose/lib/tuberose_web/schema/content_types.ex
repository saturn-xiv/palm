defmodule TuberoseWeb.Schema.ContentTypes do
  use Absinthe.Schema.Notation

  import_types(Absinthe.Type.Custom)

  @desc "Rich Media Editor"
  enum :text_editor do
    value(:textarea, as: "textarea")
    value(:quill, as: "quill")
  end

  input_object :pager do
    field(:size, non_null(:integer))
    field(:page, non_null(:integer))
  end

  object :pagination do
    field(:page, non_null(:integer))
    field(:size, non_null(:integer))
    field(:total, non_null(:integer))
    field(:has_previous, non_null(:boolean))
    field(:has_next, non_null(:boolean))
  end

  object :succeed do
    field(:created_at, non_null(:datetime))
  end

  # ---------------------------------------------------------------------------
  object :cn_gab do
    field(:code, non_null(:string))
    field(:name, non_null(:string))
  end

  object :cn_icp do
    field(:code, non_null(:string))
  end

  object :author do
    field(:name, non_null(:string))
    field(:email, non_null(:string))
  end

  object :layout_response do
    field(:favicon, non_null(:string))
    field(:title, non_null(:string))
    field(:subhead, non_null(:string))
    field(:keywords, list_of(non_null(:string)))
    field(:description, non_null(:string))
    field(:author, :author)
    field(:copyright, non_null(:string))
    field(:gab, :cn_gab)
    field(:icp, :cn_icp)
    field(:locale, non_null(:string))
    field(:languages, list_of(non_null(:string)))
  end

  object :route do
    field(:path, non_null(:string))
    field(:name, non_null(:string))
    field(:children, list_of(non_null(:route)))
  end

  # ---------------------------------------------------------------------------
  object :locale do
    field(:id, non_null(:id))
    field(:lang, non_null(:string))
    field(:code, non_null(:string))
    field(:message, non_null(:string))
    field(:updated_at, non_null(:datetime))
  end

  object :index_locale_response do
    field(:pagination, non_null(:pagination))
    field(:items, list_of(non_null(:locale)))
  end

  # ---------------------------------------------------------------------------
  object :resource do
    field(:type, non_null(:string))
    field(:id, :integer)
  end

  object :permission do
    field(:operation, non_null(:string))
    field(:resource, non_null(:resource))
  end

  object :current_user do
    field(:name, non_null(:string))
    field(:avatar, non_null(:string))
    field(:is_administrator, non_null(:boolean))
    field(:is_root, non_null(:boolean))
    field(:roles, list_of(:string))
    field(:permissions, list_of(:permission))
    field(:has_wechat_mini_program, non_null(:boolean))
    field(:has_wechat_oauth2, non_null(:boolean))
    field(:has_google, non_null(:boolean))
    field(:provider_type, non_null(:user_provider_type))
    field(:lang, non_null(:string))
    field(:timezone, non_null(:string))
  end

  object :current_email_user do
    field(:nickname, non_null(:string))
    field(:email, non_null(:string))
  end

  object :user_sign_in_response do
    field(:token, non_null(:string))
    field(:user, non_null(:current_user))
  end

  enum :user_provider_type do
    value(:email, as: "email")
    value(:phone, as: "phone")
    value(:wechat_mini_program, as: "wechat-mini-program")
    value(:wechat_oauth2, as: "wechat-oauth2")
    value(:google, as: "google")
  end

  enum :log_level do
    value(:debug, as: "debug")
    value(:info, as: "info")
    value(:warning, as: "warning")
    value(:error, as: "error")
  end

  object :log do
    field(:id, non_null(:id))
    field(:plugin, non_null(:string))
    field(:ip, non_null(:string))
    field(:level, non_null(:log_level))
    field(:resource_type, non_null(:string))
    field(:resource_id, :integer)
    field(:message, non_null(:string))
    field(:created_at, non_null(:datetime))
  end

  object :index_log_response do
    field(:pagination, non_null(:pagination))
    field(:items, list_of(:log))
  end

  # ---------------------------------------------------------------------------
  enum :leave_word_status do
    value(:pending, as: "pending")
    value(:closed, as: "closed")
  end

  object :leave_word do
    field(:id, non_null(:id))
    field(:lang, non_null(:string))
    field(:body, non_null(:string))
    field(:editor, non_null(:text_editor))
    field(:status, non_null(:leave_word_status))
    field(:updated_at, non_null(:datetime))
  end

  object :index_leave_word_response do
    field(:pagination, non_null(:pagination))
    field(:items, list_of(non_null(:leave_word)))
  end

  # ---------------------------------------------------------------------------

  object :attachment do
    field(:id, non_null(:id))
    field(:title, non_null(:string))
    field(:bucket, non_null(:string))
    field(:object, non_null(:string))
    field(:size, non_null(:integer))
    field(:content_type, non_null(:string))
    field(:uploaded_at, :datetime)
    field(:deleted_at, :datetime)
    field(:updated_at, non_null(:datetime))
  end

  object :index_attachment_response do
    field(:pagination, non_null(:pagination))
    field(:items, list_of(non_null(:attachment)))
  end

  object :show_attachment_response do
    field(:item, non_null(:attachment))
    field(:url, :string)
  end

  object :upload_attachment_url_response do
    field(:id, non_null(:id))
    field(:bucket, non_null(:string))
    field(:object, non_null(:string))
    field(:url, non_null(:string))
  end

  # ---------------------------------------------------------------------------
end
