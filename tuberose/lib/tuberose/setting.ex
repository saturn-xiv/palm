defmodule Tuberose.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field(:user_id, :integer)
    field(:key, :string)
    field(:value, :binary)

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:user_id, :key, :value, :deleted_at])
    |> validate_required([:key])
  end
end
