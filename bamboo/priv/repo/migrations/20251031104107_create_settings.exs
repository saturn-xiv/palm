defmodule Bamboo.Repo.Migrations.CreateSettings do
  use Ecto.Migration

  def change do
    create table(:settings) do
      add :key, :string, null: false, size: 255
      add :value, :binary, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime)
    end

    create unique_index(:settings, [:key])
  end
end
