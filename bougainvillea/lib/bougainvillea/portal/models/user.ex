defmodule Bougainvillea.Portal.Models.User do
  use Ecto.Schema
  import Ecto.Changeset

  schema "users" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(user, attrs) do
    user
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
