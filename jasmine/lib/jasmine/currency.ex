defmodule Jasmine.Currency do
  use Ecto.Schema
  import Ecto.Changeset

  schema "currencies" do
    field :code, :string
    field :number, :string
    field :name, :string
    field :country, :string
    field :units, :integer
    field :created_at, :utc_datetime_usec
  end

  @doc false
  def changeset(currency, attrs) do
    currency
    |> cast(attrs, [:code, :number, :name, :country, :units, :created_at])
    |> validate_required([:code, :number, :name, :country, :units])
  end
end
