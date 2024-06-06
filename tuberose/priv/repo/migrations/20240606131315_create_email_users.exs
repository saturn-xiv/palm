defmodule Tuberose.Repo.Migrations.CreateEmailUsers do
  use Ecto.Migration

  def change do
    create table(:email_users) do
      add(:user_id, references(:users), null: false)
      add(:real_name, :string, size: 63, null: false)
      add(:nickname, :string, size: 63, null: false)
      add(:email, :string, size: 127, null: false)
      add(:password, :bytea, null: false)
      add(:salt, :bytea, null: false)
      add(:avatar, :string, size: 127, null: false)
      add(:confirmed_at, :utc_datetime)
      add(:deleted_at, :utc_datetime)
      add(:version, :integer, default: 0, null: false)

      timestamps(type: :utc_datetime)
    end

    create(unique_index(:email_users, [:email]))
    create(unique_index(:email_users, [:nickname]))
    create(index(:email_users, [:real_name]))
  end
end
