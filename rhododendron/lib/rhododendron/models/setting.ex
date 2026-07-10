defmodule Rhododendron.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
