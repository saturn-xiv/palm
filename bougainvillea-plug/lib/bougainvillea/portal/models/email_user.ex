defmodule Bougainvillea.Portal.Models.EmailUser do
  use Ecto.Schema

  schema "email_users" do
    field :sn, :string
    field :name, :string
    field :email, :string
    field :password, :string
    field :avatar, :string
    field :confirmed_at, :utc_datetime_usec
    field :locked_at, :utc_datetime_usec
    field :deleted_at, :utc_datetime_usec
    field :version, :integer
    field :updated_at, :utc_datetime_usec
    field :created_at, :utc_datetime_usec

    belongs_to :user, Bougainvillea.Portal.Models.User
  end
end
