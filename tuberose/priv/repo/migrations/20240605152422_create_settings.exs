defmodule Tuberose.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add(:user_id, :bigint)
      add(:key, :string, null: false)
      add(:value, :bytea, null: false)
      add(:deleted_at, :utc_datetime)

      timestamps(type: :utc_datetime)
    end

    create(index(:settings, [:key]))
  end
end
