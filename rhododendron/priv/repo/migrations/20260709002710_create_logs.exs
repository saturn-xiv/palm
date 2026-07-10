defmodule Rhododendron.Repo.Migrations.CreateLogs do
  use Ecto.Migration

  def change do
    create table(:logs) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :plugin, :string, null: false, size: 15
      add :level, :string, null: false, size: 7
      add :ip, :string, null: false, size: 45
      add :message, :text, null: false
      add :resource, :string, null: false, size: 255
      add :resource_id, :integer

      timestamps(updated_at: false, type: :utc_datetime_usec)
    end

    create index(:logs, [:plugin])
    create index(:logs, [:level])
    create index(:logs, [:ip])
    create index(:logs, [:resource])
  end
end
