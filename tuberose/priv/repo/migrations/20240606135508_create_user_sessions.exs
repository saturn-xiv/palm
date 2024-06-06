defmodule Tuberose.Repo.Migrations.CreateUserSessions do
  use Ecto.Migration

  def change do
    create table(:user_sessions) do
      add(:user_id, references(:users), null: false)
      add(:uid, :string, size: 36, null: false)
      add(:provider_type, :string, size: 31, null: false)
      add(:provider_id, :bigint, null: false)
      add(:ip, :string, size: 45, null: false)
      add(:expires_at, :utc_datetime, null: false)
      add(:created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP"))
    end

    create(unique_index(:user_sessions, [:uid]))
    create(index(:user_sessions, [:provider_type]))
    create(index(:user_sessions, [:ip]))
  end
end
