defmodule Tuberose.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    belongs_to(:user, Tuberose.User)

    field(:key, :string)
    field(:value, :binary)
    field(:salt, :binary)
    field(:version, :integer)

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:user_id, :key, :value, :salt, :version])
    |> validate_required([:key, :value])
  end
end
