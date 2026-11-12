defmodule Bougainvillea.Repo.Migrations.Portal.CreateVotes do
  use Ecto.Migration

  def change do
    create table(:votes) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :ip, :string, size: 45, null: false
      add :stars, :integer, null: false
      add :comment, :map, null: false, default: fragment("'[]'::jsonb")
      add :published_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:votes, [:ip])
    create index(:votes, [:comment], using: :gin)
  end
end
