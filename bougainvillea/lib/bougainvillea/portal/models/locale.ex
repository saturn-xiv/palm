defmodule Bougainvillea.Portal.Models.Locale do
  use Ecto.Schema
  import Ecto.Changeset

  schema "locales" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(locale, attrs) do
    locale
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
