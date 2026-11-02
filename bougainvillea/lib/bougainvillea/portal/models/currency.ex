defmodule Bougainvillea.Portal.Models.Currency do
  use Ecto.Schema

  schema "currencies" do
    field :name, :string
    field :code, :string
    field :country, :string
    field :number, :integer
    field :units, :integer
    field :fund, :boolean
    field :created_at, :utc_datetime_usec
  end
end
