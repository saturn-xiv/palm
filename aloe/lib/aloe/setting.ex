defmodule Aloe.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field(:key, :string)
    field(:value, :binary)
    field(:encode, :boolean)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:key, :value, :encode])
    |> validate_required([:key, :value, :encode])
  end
end
