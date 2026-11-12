defmodule Bougainvillea.Portal.Models.Vote do
  use Ecto.Schema
  import Ecto.Changeset

  schema "votes" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(vote, attrs) do
    vote
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
