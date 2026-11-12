defmodule Bougainvillea.Repo.Migrations.Portal.CreateWechatOauth2User do
  use Ecto.Migration

  def change do
    create table(:wechat_oauth2_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 36, null: false
      add :union_id, :string, size: 63, null: false
      add :app_id, :string, size: 63, null: false
      add :open_id, :string, size: 63, null: false
      add :nickname, :string, size: 63, null: false
      add :head_img_url, :string, size: 127
      add :privilege, :map, null: false, default: fragment("'[]'::jsonb")
      add :access_token, :string, size: 63, null: false
      add :expires_in, :utc_datetime_usec, null: false
      add :refresh_token, :string, size: 63, null: false
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:wechat_oauth2_users, [:nickname])
    create index(:wechat_oauth2_users, [:privilege], using: :gin)
    create unique_index(:wechat_oauth2_users, [:app_id, :open_id])
    create unique_index(:wechat_oauth2_users, [:union_id])
    create unique_index(:wechat_oauth2_users, [:sn])
  end
end
