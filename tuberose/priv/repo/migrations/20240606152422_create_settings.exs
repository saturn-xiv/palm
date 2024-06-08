defmodule Tuberose.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add(:user_id, references(:users))
      add(:key, :string, null: false)
      add(:value, :binary, null: false, redact: true)
      add(:salt, :bit, size: 32, redact: true)
      add(:version, :integer, default: 0, null: false)

      timestamps(type: :utc_datetime_usec)
    end

    create(unique_index(:settings, [:key], where: "user_id IS NULL"))
    create(unique_index(:settings, [:user_id, :key], where: "user_id IS NOT NULL"))
  end
end
