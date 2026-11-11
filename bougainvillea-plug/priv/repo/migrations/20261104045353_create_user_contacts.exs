defmodule Bougainvillea.Repo.Migrations.CreateUserContacts do
  use Ecto.Migration

  def change do
    create table(:user_contacts) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :key, :string, size: 45, null: false
      add :value, :map, null: false, default: fragment("'{}'::jsonb")
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:user_contacts, [:value], using: :gin)
    create unique_index(:user_contacts, [:user_id, :key])
  end
end
