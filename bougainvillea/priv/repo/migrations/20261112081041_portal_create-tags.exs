defmodule Bougainvillea.Repo.Migrations.Portal.CreateTags do
  use Ecto.Migration

  def change do
    create table(:tags) do
      add :code, :string, size: 31, null: false
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:tags, [:code])
  end
end
