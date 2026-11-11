defmodule Bougainvillea.Repo.Migrations.CreateWechatMiniProgramUsers do
  use Ecto.Migration

  def change do
    create table(:wechat_mini_program_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 36, null: false
      add :union_id, :string, size: 63, null: false
      add :app_id, :string, size: 63, null: false
      add :open_id, :string, size: 63, null: false
      add :nickname, :string, size: 63
      add :avatar_url, :string, size: 127
      add :session_key, :string, size: 127, null: false
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:wechat_mini_program_users, [:nickname], where: "nickname IS NULL")
    create unique_index(:wechat_mini_program_users, [:app_id, :open_id])
    create unique_index(:wechat_mini_program_users, [:union_id])
    create unique_index(:wechat_mini_program_users, [:sn])
  end
end
