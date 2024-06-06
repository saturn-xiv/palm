defmodule Tuberose.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    belongs_to(:user, Tuberose.User)

    field(:key, :string)
    field(:value, :binary)
    field(:version, :integer)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:user_id, :key, :value, :deleted_at, :version])
    |> validate_required([:key])
  end
end
