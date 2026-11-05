defmodule Bougainvillea.Portal.Models.Log do
  use Ecto.Schema

  schema "logs" do
    field :plugin, :string
    field :level, :string
    field :ip, :string
    field :message, :string
    field :created_at, :utc_datetime_usec

    belongs_to :user, Bougainvillea.Portal.Models.User
  end
end
