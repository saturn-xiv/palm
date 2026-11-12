defmodule Bougainvillea.Portal.Models.User.Ban do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_bans" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(user_bans, attrs) do
    user_bans
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
