defmodule Rhododendron.Repo.Migrations.CreateWechatMiniProgramUsers do
  use Ecto.Migration

  def change do
    create table(:wechat_mini_program_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :union_id, :string, null: false, size: 127
      add :app_id, :string, null: false, size: 63
      add :open_id, :string, null: false, size: 127
      add :nickname, :string, size: 63
      add :avatar_url, :string, size: 127
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:wechat_mini_program_users, [:union_id])
    create unique_index(:wechat_mini_program_users, [:app_id, :open_id])
    create index(:wechat_mini_program_users, [:app_id])
    create index(:wechat_mini_program_users, [:open_id])
    create index(:wechat_mini_program_users, [:nickname], where: "nickname IS NOT NULL")
  end
end
