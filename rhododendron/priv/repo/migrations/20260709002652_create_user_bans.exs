defmodule Rhododendron.Repo.Migrations.CreateUserBans do
  use Ecto.Migration

  def change do
    create table(:user_bans) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :creator_id, references(:users, on_delete: :delete_all), null: false
      add :ip, :string, null: false, size: 45
      add :reason, :string, null: false, size: 511
      add :expired_at, :utc_datetime_usec, null: false
      add :deleted_at, :utc_datetime_usec

      timestamps(updated_at: false, type: :utc_datetime_usec)
    end

    create index(:user_bans, [:ip])
    create index(:user_bans, [:reason])
  end
end
