defmodule Bougainvillea.Repo.Migrations.Portal.CreateUserContacts do
  use Ecto.Migration

  def change do
    create table(:user_contacts) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :key, :string, size: 45, null: false
      add :value, :map, null: false, default: fragment("'{}'::jsonb")
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:user_contacts, [:value], using: :gin)
    create unique_index(:user_contacts, [:user_id, :key])
  end
end
