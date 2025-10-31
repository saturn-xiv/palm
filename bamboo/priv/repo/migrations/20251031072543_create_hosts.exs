defmodule Bamboo.Repo.Migrations.CreateHosts do
  use Ecto.Migration

  def change do
    create table(:hosts) do
      add :name, :string, size: 63
      add :mac, :string, null: false, size: 17
      add :ip, :string, null: false, size: 39
      add :fixed, :boolean, null: false, default: false
      add :member_id, references(:members)
      add :version, :integer, null: false, default: 0
      add :deleted_at, :utc_datetime

      timestamps(type: :utc_datetime)
    end

    create unique_index(:hosts, [:mac])
    create index(:hosts, [:ip])
    create index(:hosts, [:name], where: "name IS NOT NULL")
  end
end
