defmodule Rhododendron.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field :user_id, :integer
    field :key, :string
    field :value, :binary
    field :salt, :binary

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:user_id, :key, :value, :salt])
    |> validate_required([:key, :value])
  end
end
