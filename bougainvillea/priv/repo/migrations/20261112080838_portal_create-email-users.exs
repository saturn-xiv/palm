defmodule Bougainvillea.Repo.Migrations.Portal.CreateEmailUsers do
  use Ecto.Migration

  def change do
    create table(:email_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 36, null: false
      add :name, :string, size: 31, null: false
      add :email, :string, size: 63, null: false
      add :password, :string, size: 255, null: false
      add :avatar, :string, size: 127, null: false
      add :confirmed_at, :utc_datetime_usec
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:email_users, [:name])
    create unique_index(:email_users, [:email])
    create unique_index(:email_users, [:sn])
  end
end
