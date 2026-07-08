defmodule Rhododendron.Currency do
  use Ecto.Schema
  import Ecto.Changeset

  schema "currencies" do
    field :name, :string
    field :code, :string
    field :country, :string
    field :number, :integer
    field :units, :integer
    field :is_fund, :boolean

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(currency, attrs) do
    currency
    |> cast(attrs, [:name, :code, :country, :number, :units, :is_fund])
    |> validate_required([:name, :code, :country, :number])
  end
end
