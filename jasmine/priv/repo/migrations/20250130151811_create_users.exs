defmodule Jasmine.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def up do
    create table(:users) do
      add :uid, :string, size: 36, null: false
      add :lang, :string, size: 15, null: false, default: "en-US"
      add :timezone, :string, size: 31, null: false, default: "UTC"
      add :sign_in_count, :integer, null: false, default: 0
      add :current_sign_in_at, :utc_datetime
      add :current_sign_in_ip, :string, size: 45
      add :last_sign_in_at, :utc_datetime
      add :last_sign_in_ip, :string, size: 45
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:users, [:uid], unique: true)
    create index(:users, [:lang])
    create index(:users, [:timezone])
    create index(:users, [:current_sign_in_ip], where: "current_sign_in_ip IS NOT NULL")
    create index(:users, [:last_sign_in_ip], where: "last_sign_in_ip IS NOT NULL")

    create table(:logs) do
      add :user_id, :bigint, null: false
      add :plugin, :string, size: 31, null: false
      add :ip, :string, size: 45, null: false
      add :level, :string, size: 8, null: false
      add :resource_type, :string, size: 255, null: false
      add :resource_id, :bigint
      add :message, :text, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:logs, [:plugin])
    create index(:logs, [:ip])
    create index(:logs, [:level])
    create index(:logs, [:resource_type])

    create table(:sessions) do
      add :user_id, :bigint, null: false
      add :uid, :string, size: 36, null: false
      add :name, :string, size: 63, null: false
      add :provider_type, :string, size: 31, null: false
      add :provider_id, :bigint, null: false
      add :ip, :string, size: 45, null: false
      add :expires_at, :utc_datetime_usec, null: false
      add :deleted_at, :utc_datetime
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:sessions, [:uid], unique: true)
    create index(:sessions, [:name])
    create index(:sessions, [:provider_type])
    create index(:sessions, [:ip])
  end

  def down do
    drop table(:sessions)
    drop table(:logs)
    drop table(:users)
  end
end
