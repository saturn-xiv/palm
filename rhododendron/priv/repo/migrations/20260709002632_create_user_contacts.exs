defmodule Rhododendron.Repo.Migrations.CreateUserContacts do
  use Ecto.Migration

  def change do
    create table(:user_contacts) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :key, :string, null: false, size: 255
      add :value, :binary, null: false
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:user_contacts, [:user_id, :key])
    create index(:user_contacts, [:key])
  end
end
