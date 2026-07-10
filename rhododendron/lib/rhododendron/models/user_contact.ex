defmodule Rhododendron.UserContact do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_contacts" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(user_contact, attrs) do
    user_contact
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
