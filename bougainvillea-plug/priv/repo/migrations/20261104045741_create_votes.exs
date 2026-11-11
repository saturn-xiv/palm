defmodule Bougainvillea.Repo.Migrations.CreateVotes do
  use Ecto.Migration

  def change do
    create table(:votes) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :ip, :string, size: 45, null: false
      add :stars, :integer, null: false
      add :comment, :text, null: false
      add :comment_editor, :string, size: 15, null: false
      add :published_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime_usec, null: false
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:votes, [:ip])
  end
end
