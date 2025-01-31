defmodule Jasmine.Repo.Migrations.CreateWechatUsers do
  use Ecto.Migration

  def up do
    create table(:wechat_mini_program_users) do
      add :user_id, :bigint, null: false
      add :union_id, :string, size: 127, null: false
      add :app_id, :string, size: 63, null: false
      add :open_id, :string, size: 127, null: false
      add :nickname, :string, size: 63
      add :avatar_url, :string, size: 255
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:wechat_mini_program_users, [:union_id], unique: true)
    create index(:wechat_mini_program_users, [:open_id, :app_id], unique: true)
    create index(:wechat_mini_program_users, [:open_id])
    create index(:wechat_mini_program_users, [:app_id])
    create index(:wechat_mini_program_users, [:nickname], where: "nickname IS NOT NULL")

    create table(:wechat_oauth2_users) do
      add :user_id, :bigint, null: false
      add :union_id, :string, size: 127, null: false
      add :app_id, :string, size: 63, null: false
      add :open_id, :string, size: 127, null: false
      add :nickname, :string, size: 63, null: false
      add :sex, :integer, null: false
      add :city, :string, size: 63, null: false
      add :province, :string, size: 63, null: false
      add :country, :string, size: 63, null: false
      add :head_img_url, :string, size: 255
      add :privilege, :binary, null: false
      add :lang, :string, size: 7, null: false
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:wechat_oauth2_users, [:union_id], unique: true)
    create index(:wechat_oauth2_users, [:app_id, :open_id], unique: true)
    create index(:wechat_oauth2_users, [:app_id])
    create index(:wechat_oauth2_users, [:open_id])
    create index(:wechat_oauth2_users, [:nickname])
    create index(:wechat_oauth2_users, [:city])
    create index(:wechat_oauth2_users, [:province])
    create index(:wechat_oauth2_users, [:country])
    create index(:wechat_oauth2_users, [:lang])
  end

  def down do
    drop table(:wechat_mini_program_users)
    drop table(:wechat_oauth2_users)
  end
end
