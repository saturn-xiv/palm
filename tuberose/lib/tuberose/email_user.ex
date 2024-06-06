defmodule Tuberose.EmailUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "email_users" do
    field(:user_id, :integer)
    field(:real_name, :string)
    field(:nickname, :string)
    field(:email, :string)
    field(:password, :binary)
    field(:salt, :binary)
    field(:avatar, :string)
    field(:confirmed_at, :utc_datetime)
    field(:deleted_at, :utc_datetime)
    field(:version, :integer)

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(email_user, attrs) do
    email_user
    |> cast(attrs, [
      :user_id,
      :real_name,
      :nickname,
      :email,
      :password,
      :salt,
      :avatar,
      :confirmed_at,
      :deleted_at,
      :version
    ])
    |> validate_required([:user_id, :real_name, :nickname, :email])
  end
end
