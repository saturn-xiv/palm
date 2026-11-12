defmodule Bougainvillea.Portal.Models.User.ByGoogleOauth2 do
  use Ecto.Schema
  import Ecto.Changeset

  schema "google_oauth2_users" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(google_oauth2_user, attrs) do
    google_oauth2_user
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
