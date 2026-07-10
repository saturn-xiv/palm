defmodule Rhododendron.Repo.Migrations.CreateWechatOauth2Users do
  use Ecto.Migration

  def change do
    create table(:wechat_oauth2_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :union_id, :string, null: false, size: 127
      add :app_id, :string, null: false, size: 63
      add :open_id, :string, null: false, size: 127
      add :nickname, :string, null: false, size: 63
      add :sex, :integer, null: false
      add :city, :string, null: false, size: 63
      add :province, :string, null: false, size: 63
      add :country, :string, null: false, size: 63
      add :head_img_url, :string, size: 127
      add :privilege, :binary, null: false
      add :lang, :string, null: false, size: 7
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:wechat_oauth2_users, [:union_id])
    create unique_index(:wechat_oauth2_users, [:app_id, :open_id])
    create index(:wechat_oauth2_users, [:app_id])
    create index(:wechat_oauth2_users, [:open_id])
    create index(:wechat_oauth2_users, [:nickname])
    create index(:wechat_oauth2_users, [:city])
    create index(:wechat_oauth2_users, [:province])
    create index(:wechat_oauth2_users, [:country])
    create index(:wechat_oauth2_users, [:lang])
  end
end
