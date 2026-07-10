defmodule Rhododendron.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users) do
      add :uid, :string, null: false, size: 36
      add :name, :string, size: 31
      add :avatar, :string, size: 127
      add :lang, :string, null: false, size: 15, default: "en-US"
      add :timezone, :string, null: false, size: 31, default: "UTC"
      add :sign_in_count, :integer, null: false, default: 0
      add :current_sign_in_at, :utc_datetime_usec
      add :current_sign_in_ip, :string, size: 45
      add :last_sign_in_at, :utc_datetime_usec
      add :last_sign_in_ip, :string, size: 45
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:users, [:uid])
    create index(:users, [:lang])
    create index(:users, [:timezone])
    create index(:users, [:name], where: "name IS NOT NULL")
    create index(:users, [:current_sign_in_ip], where: "current_sign_in_ip IS NOT NULL")
    create index(:users, [:last_sign_in_ip], where: "last_sign_in_ip IS NOT NULL")
  end
end
