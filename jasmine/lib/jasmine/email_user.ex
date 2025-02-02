defmodule Jasmine.EmailUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "email_users" do
    field :user_id, :integer
    field :name, :string
    field :email, :string
    field :password, :binary
    field :avatar, :string
    field :confirmed_at, :utc_datetime_usec
    field :locked_at, :utc_datetime_usec
    field :deleted_at, :utc_datetime_usec
    field :version, :integer
    field :updated_at, :utc_datetime_usec
    field :created_at, :utc_datetime_usec
  end

  @doc false
  def changeset(email_user, attrs) do
    email_user
    |> cast(attrs, [
      :user_id,
      :name,
      :email,
      :password,
      :avatar,
      :confirmed_at,
      :locked_at,
      :deleted_at,
      :version,
      :updated_at,
      :created_at
    ])
    |> validate_required([:user_id, :name, :email, :password, :avatar])
  end
end
