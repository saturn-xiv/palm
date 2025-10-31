defmodule Bamboo.Repo.Migrations.CreateMembers do
  use Ecto.Migration

  def change do
    create table(:members) do
      add :sn, :string, null: false, size: 31
      add :name, :string, null: false, size: 63
      add :memo, :text, null: false
      add :wifi_password, :string, null: false, size: 255
      add :version, :integer, null: false, default: 0
      add :deleted_at, :utc_datetime

      timestamps(type: :utc_datetime)
    end

    create unique_index(:members, [:sn])
    create index(:members, [:name])
  end
end
