defmodule Rhododendron.GoogleUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "google_users" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(google_user, attrs) do
    google_user
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
