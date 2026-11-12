defmodule Bougainvillea.Portal.Models.User.ByWechatOauth2 do
  use Ecto.Schema
  import Ecto.Changeset

  schema "wechat_oauth2_users" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(wechat_oauth2_user, attrs) do
    wechat_oauth2_user
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
