defmodule Rhododendron.EmailUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "email_users" do
    field :name, :string
    field :email, :string
    field :password, :string
    field :avatar, :string
    field :confirmed_at, :utc_datetime_usec
    field :locked_at, :utc_datetime_usec
    field :deleted_at, :utc_datetime_usec
    field :version, :integer

    belongs_to :user, Rhododendron.User

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(email_user, attrs) do
    email_user
    |> cast(attrs, [
      :name,
      :email,
      :password,
      :avatar,
      :confirmed_at,
      :locked_at,
      :deleted_at,
      :version
    ])
    |> validate_required([:name, :email, :password])
  end
end
