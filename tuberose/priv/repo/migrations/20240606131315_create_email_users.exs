defmodule Tuberose.Repo.Migrations.CreateEmailUsers do
  use Ecto.Migration

  def change do
    create table(:email_users) do
      add(:user_id, references(:users), null: false)
      add(:real_name, :string, size: 63, null: false)
      add(:nickname, :string, size: 63, null: false)
      add(:email, :string, size: 127, null: false)
      add(:password, :binary, null: false, redact: true)
      add(:salt, :binary, null: false, redact: true)
      add(:avatar, :string, size: 127, null: false)
      add(:confirmed_at, :utc_datetime_usec)
      add(:deleted_at, :utc_datetime_usec)
      add(:version, :integer, default: 0, null: false)

      timestamps(type: :utc_datetime_usec)
    end

    create(unique_index(:email_users, [:email]))
    create(unique_index(:email_users, [:nickname]))
    create(index(:email_users, [:real_name]))
  end
end
