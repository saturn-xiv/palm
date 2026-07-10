defmodule Rhododendron.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add :user_id, references(:users, on_delete: :delete_all)
      add :key, :string, null: false, size: 255
      add :salt, :binary
      add :value, :binary, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:settings, [:key, :user_id], where: "user_id IS NOT NULL")
    create unique_index(:settings, [:key], where: "user_id IS NULL")
  end
end
