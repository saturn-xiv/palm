defmodule Bougainvillea.Repo.Migrations.CreateLavenderTasks do
  use Ecto.Migration

  def change do
    create table(:lavender_tasks) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 36, null: false
      add :work_dir, :string, size: 127, null: false
      add :args, :map, null: false
      add :script, :text, null: false
      add :launched_at, :utc_datetime_usec
      add :finished_at, :utc_datetime_usec
      add :status_code, :integer
      add :stdout, :text
      add :stderr, :text
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:lavender_tasks, [:work_dir])
    create unique_index(:lavender_tasks, [:sn])
  end
end
