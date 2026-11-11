defmodule Bougainvillea.Portal.Models.Locale do
  use Ecto.Schema

  schema "locales" do
    field :lang, :string
    field :code, :string
    field :message, :string
    field :version, :integer
    field :updated_at, :utc_datetime_usec
    field :created_at, :utc_datetime_usec
  end
end
