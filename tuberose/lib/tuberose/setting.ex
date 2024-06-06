defmodule Tuberose.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    belongs_to(:user, Tuberose.User)

    field(:key, :string)
    field(:value, :binary)
    field(:deleted_at, :utc_datetime)
    field(:version, :integer)

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:user_id, :key, :value, :deleted_at, :version])
    |> validate_required([:key])
  end
end
