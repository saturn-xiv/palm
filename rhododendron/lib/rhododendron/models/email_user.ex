defmodule Rhododendron.EmailUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "email_users" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(email_user, attrs) do
    email_user
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
