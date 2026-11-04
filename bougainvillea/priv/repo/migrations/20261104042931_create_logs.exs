defmodule Bougainvillea.Repo.Migrations.CreateLogs do
  use Ecto.Migration

  def change do
    create table(:logs) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :plugin, :string, size: 31, null: false
      add :level, :string, size: 7, null: false
      add :ip, :string, size: 45, null: false
      add :message, :text, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:logs, [:plugin])
    create index(:logs, [:level])
    create index(:logs, [:ip])
  end
end
