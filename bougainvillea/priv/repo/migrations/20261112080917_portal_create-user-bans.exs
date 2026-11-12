defmodule Bougainvillea.Repo.Migrations.Portal.CreateUserBans do
  use Ecto.Migration

  def change do
    create table(:user_bans) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :creator_id, references(:users, on_delete: :delete_all), null: false
      add :ip, :string, size: 45, null: false
      add :reason, :text, null: false
      add :expired_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec

      timestamps(type: :utc_datetime_usec, updated_at: false)
    end

    create index(:user_bans, [:ip])
  end
end
