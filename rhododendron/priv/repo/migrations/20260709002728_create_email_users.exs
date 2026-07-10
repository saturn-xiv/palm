defmodule Rhododendron.Repo.Migrations.CreateEmailUsers do
  use Ecto.Migration

  def change do
    create table(:email_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :name, :string, null: false, size: 31
      add :email, :string, null: false, size: 63
      add :password, :string, null: false, size: 127
      add :avatar, :string, null: false, size: 127
      add :confirmed_at, :utc_datetime_usec
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:email_users, [:email])
    create index(:email_users, [:name])
  end
end
