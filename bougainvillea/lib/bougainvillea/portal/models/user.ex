defmodule Bougainvillea.Portal.Models.User do
  use Ecto.Schema

  schema "users" do
    field :sn, :string
    field :name, :string
    field :avatar, :string
    field :lang, :string
    field :timezone, :string
    field :sign_in_count, :integer
    field :current_sign_in_at, :utc_datetime_usec
    field :current_sign_in_ip, :string
    field :last_sign_in_at, :utc_datetime_usec
    field :last_sign_in_ip, :string
    field :locked_at, :utc_datetime_usec
    field :deleted_at, :utc_datetime_usec
    field :version, :integer
    field :updated_at, :utc_datetime_usec
    field :created_at, :utc_datetime_usec

    has_many :logs, Bougainvillea.Portal.Models.Log
    has_many :email_users, Bougainvillea.Portal.Models.EmailUser
  end
end
