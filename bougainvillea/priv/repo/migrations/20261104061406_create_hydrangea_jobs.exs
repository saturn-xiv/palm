defmodule Bougainvillea.Repo.Migrations.CreateLavenderTasks do
  use Ecto.Migration

  def change do
    create table(:hydrangea_jobs) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 36, null: false
      add :name, :string, size: 63, null: false
      add :summary, :string, size: 511, null: false
      add :version, :string, size: 15, null: false
      add :working_dir, :string, size: 127, null: false
      add :args, :map, null: false
      add :script, :text, null: false
      add :launched_at, :utc_datetime_usec, null: false
      add :elapsed, :utc_datetime_usec, null: false
      add :status_code, :integer, null: false
      add :stdout, :text, null: false
      add :stderr, :text, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:hydrangea_jobs, [:name])
    create index(:hydrangea_jobs, [:summary])
    create index(:hydrangea_jobs, [:version])
    create index(:hydrangea_jobs, [:working_dir])
    create unique_index(:hydrangea_jobs, [:sn])
  end
end
