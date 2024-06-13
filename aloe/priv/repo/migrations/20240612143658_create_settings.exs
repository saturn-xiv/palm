defmodule Aloe.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add(:key, :string, size: 255, null: false)
      add(:value, :binary, null: false, redact: true)
      add(:encode, :boolean, null: false)

      timestamps(type: :utc_datetime_usec)
    end

    create(unique_index(:settings, [:key]))
  end
end
