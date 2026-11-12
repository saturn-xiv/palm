defmodule Bougainvillea.Portal.Models.Setting do
  use Ecto.Schema
  import Ecto.Changeset

  schema "settings" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(setting, attrs) do
    setting
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
