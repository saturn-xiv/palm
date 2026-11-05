defmodule Bougainvillea.Hydrangea.Models.Job do
  use Ecto.Schema

  schema "hydrangea_jobs" do
    field :lang, :string
    field :code, :string
    field :message, :string
    field :version, :integer
    field :updated_at, :utc_datetime_usec
    field :created_at, :utc_datetime_usec
  end
end
