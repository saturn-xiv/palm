defmodule Bougainvillea.Portal.Models.Tag do
  use Ecto.Schema
  import Ecto.Changeset

  schema "tags" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(tag, attrs) do
    tag
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
