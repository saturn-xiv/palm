defmodule Jasmine.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def up do
    create table(:settings) do
      add :user_id, :integer
      add :key, :string, size: 255, null: false
      add :value, :binary, null: false
      add :nonce, :binary
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:settings, [:key, :user_id], unique: true, where: "user_id IS NOT NULL")
    create index(:settings, [:key], unique: true, where: "user_id IS NULL")
  end

  def down do
    drop table(:settings)
  end
end
