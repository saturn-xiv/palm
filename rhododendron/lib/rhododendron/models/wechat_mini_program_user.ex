defmodule Rhododendron.WechatMiniProgramUser do
  use Ecto.Schema
  import Ecto.Changeset

  schema "wechat_mini_program_users" do
    field :name, :string

    timestamps(type: :utc_datetime_usec)
  end

  @doc false
  def changeset(wechat_mini_program_user, attrs) do
    wechat_mini_program_user
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
