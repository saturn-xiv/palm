defmodule Bougainvillea.Repo.Migrations.Portal.CreateLogs do
  use Ecto.Migration

  def change do
    create table(:logs) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :plugin, :string, size: 31, null: false
      add :level, :string, size: 7, null: false
      add :ip, :string, size: 45, null: false
      add :message, :text, null: false

      timestamps(type: :utc_datetime_usec, updated_at: false)
    end

    create index(:logs, [:plugin])
    create index(:logs, [:level])
    create index(:logs, [:ip])
  end
end
