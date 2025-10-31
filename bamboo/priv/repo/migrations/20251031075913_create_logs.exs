defmodule Bamboo.Repo.Migrations.CreateLogs do
  use Ecto.Migration

  def change do
    create table(:logs) do
      add :user_id, references(:users), null: false
      add :level, :string, null: false, size: 15
      add :message, :text, null: false

      timestamps(type: :utc_datetime, updated_at: false)
    end

    create index(:logs, [:level])
  end
end
