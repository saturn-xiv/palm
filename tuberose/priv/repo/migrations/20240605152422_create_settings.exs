defmodule Tuberose.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add(:user_id, :bigint)
      add(:key, :string, null: false)
      add(:value, :bytea, null: false)
      add(:deleted_at, :utc_datetime)
      add(:version, :integer, default: 0, null: false)

      timestamps(type: :utc_datetime)
    end

    create(unique_index(:settings, [:key], where: "user_id IS NULL"))
    create(unique_index(:settings, [:user_id, :key], where: "user_id IS NOT NULL"))
  end
end
