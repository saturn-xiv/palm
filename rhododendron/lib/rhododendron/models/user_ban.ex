defmodule Rhododendron.UserBan do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_bans" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(user_ban, attrs) do
    user_ban
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
