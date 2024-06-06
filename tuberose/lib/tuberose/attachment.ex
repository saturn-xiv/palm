defmodule Tuberose.Attachment do
  use Ecto.Schema
  import Ecto.Changeset

  schema "attachments" do
    belongs_to(:user, Tuberose.User)

    field(:bucket, :string)
    field(:object, :string)
    field(:title, :string)
    field(:size, :integer)
    field(:content_type, :string)
    field(:uploaded_at, :utc_datetime_usec)
    field(:deleted_at, :utc_datetime_usec)
    field(:version, :integer)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(attachment, attrs) do
    attachment
    |> cast(attrs, [
      :user_id,
      :bucket,
      :object,
      :title,
      :size,
      :content_type,
      :uploaded_at,
      :deleted_at,
      :version
    ])
    |> validate_required([:user_id, :bucket, :object, :title, :content_type])
  end
end
