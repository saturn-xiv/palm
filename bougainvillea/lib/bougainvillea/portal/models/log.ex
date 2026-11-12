defmodule Bougainvillea.Portal.Models.Log do
  use Ecto.Schema
  import Ecto.Changeset

  schema "logs" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(log, attrs) do
    log
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
