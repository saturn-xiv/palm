defmodule Rhododendron.Policy do
  use Ecto.Schema
  import Ecto.Changeset

  schema "policies" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(policy, attrs) do
    policy
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
