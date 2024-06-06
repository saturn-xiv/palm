defmodule Tuberose.User do
  use Ecto.Schema
  import Ecto.Changeset

  schema "users" do
    field(:uid, :string)
    field(:name, :string)
    field(:avatar, :string)
    field(:lang, :string)
    field(:timezone, :string)
    field(:sign_in_count, :integer)
    field(:current_sign_in_at, :utc_datetime_usec)
    field(:current_sign_in_ip, :string)
    field(:last_sign_in_at, :utc_datetime_usec)
    field(:last_sign_in_ip, :string)
    field(:locked_at, :utc_datetime_usec)
    field(:deleted_at, :utc_datetime_usec)
    field(:version, :integer)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(user, attrs) do
    user
    |> cast(attrs, [
      :uid,
      :name,
      :avatar,
      :lang,
      :timezone,
      :sign_in_count,
      :current_sign_in_at,
      :current_sign_in_ip,
      :last_sign_in_at,
      :last_sign_in_ip,
      :locked_at,
      :deleted_at,
      :version
    ])
    |> validate_required([:uid, :lang, :timezone])
  end
end
