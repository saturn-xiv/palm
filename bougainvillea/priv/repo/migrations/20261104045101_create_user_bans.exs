defmodule Bougainvillea.Repo.Migrations.CreateUserBans do
  use Ecto.Migration

  def change do
    create table(:user_bans) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :creator_id, references(:users, on_delete: :delete_all), null: false
      add :ip, :string, size: 45, null: false
      add :reason, :text, null: false
      add :expired_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :created_at, :utc_datetime_usec, null: false, default: fragment("now()")
    end

    create index(:user_bans, [:ip])
  end
end
