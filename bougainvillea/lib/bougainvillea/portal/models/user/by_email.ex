defmodule Bougainvillea.Portal.Models.User.ByEmail do
  use Ecto.Schema
  import Ecto.Changeset

  schema "email_users" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(email_user, attrs) do
    email_user
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
