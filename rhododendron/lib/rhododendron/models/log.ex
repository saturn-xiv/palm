defmodule Rhododendron.Log do
  use Ecto.Schema
  import Ecto.Changeset

  schema "logs" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(log, attrs) do
    log
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
