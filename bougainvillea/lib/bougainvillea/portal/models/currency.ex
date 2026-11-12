defmodule Bougainvillea.Portal.Models.Currency do
  use Ecto.Schema
  import Ecto.Changeset

  schema "currencies" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(currency, attrs) do
    currency
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
