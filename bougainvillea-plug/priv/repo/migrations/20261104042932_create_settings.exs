defmodule Bougainvillea.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add :user_id, references(:users, on_delete: :delete_all)
      add :key, :string, size: 255, null: false
      add :value, :binary, null: false
      add :associated_data, :binary, null: false
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create unique_index(:settings, [:key], where: "user_id IS NULL")
    create unique_index(:settings, [:key, :user_id], where: "user_id IS NOT NULL")
  end
end
