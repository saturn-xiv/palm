defmodule Bamboo.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users) do
      add :name, :string, null: false, size: 31
      add :password, :string, null: false, size: 255
      add :version, :integer, null: false, default: 1

      timestamps(type: :utc_datetime)
    end

    create unique_index(:users, [:name])
  end
end
