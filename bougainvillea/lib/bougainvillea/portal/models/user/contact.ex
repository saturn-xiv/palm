defmodule Bougainvillea.Portal.Models.User.Contact do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_contacts" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(user_contact, attrs) do
    user_contact
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
