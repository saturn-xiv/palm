defmodule Tuberose.UserSession do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_sessions" do
    belongs_to(:user, Tuberose.User)

    field(:uid, :string)
    field(:provider_type, :string)
    field(:provider_id, :integer)
    field(:ip, :string)
    field(:expires_at, :utc_datetime_usec)
    field(:created_at, :utc_datetime_usec)
  end

  @doc false
  def changeset(user_session, attrs) do
    user_session
    |> cast(attrs, [:user_id, :uid, :provider_type, :provider_id, :ip, :expires_at, :created_at])
    |> validate_required([:user_id, :uid, :provider_type, :provider_id, :ip])
  end
end
