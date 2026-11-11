defmodule Bougainvillea.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users) do
      add :sn, :string, size: 36, null: false
      add :name, :string, size: 31, null: false
      add :avatar, :string, size: 127
      add :lang, :string, size: 7, null: false, default: "en-US"
      add :timezone, :string, size: 31, null: false, default: "UTC"
      add :sign_in_count, :integer, null: false, default: 0
      add :current_sign_in_at, :utc_datetime_usec
      add :current_sign_in_ip, :string, size: 45
      add :last_sign_in_at, :utc_datetime_usec
      add :last_sign_in_ip, :string, size: 45
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:users, [:name])
    create index(:users, [:lang])
    create index(:users, [:timezone])
    create index(:users, [:current_sign_in_ip], where: "current_sign_in_ip IS NOT NULL")
    create index(:users, [:last_sign_in_ip], where: "last_sign_in_ip IS NOT NULL")
    create unique_index(:users, [:sn])
  end
end
