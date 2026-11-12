defmodule Bougainvillea.Portal.Models.Category do
  use Ecto.Schema
  import Ecto.Changeset

  schema "categories" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(category, attrs) do
    category
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
