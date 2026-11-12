defmodule Bougainvillea.Repo.Migrations.Portal.CreateGoogleOauth2User do
  use Ecto.Migration

  def change do
    create table(:google_oauth2_users) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 36, null: false
      add :client_id, :string, size: 63, null: false
      add :sub, :string, size: 63, null: false
      add :email, :string, size: 127, null: false
      add :email_verified, :boolean, null: false
      add :hosted_domain, :string, size: 63
      add :given_name, :string, size: 31
      add :gender, :string, size: 63
      add :link, :string, size: 127
      add :family_name, :string, size: 31
      add :name, :string, size: 63
      add :picture, :string, size: 127
      add :locale, :string, size: 15
      add :token, :string, size: 127, null: false
      add :refresh_token, :string, size: 127, null: false
      add :granted_scopes, :map, null: false, default: fragment("'[]'::jsonb")
      add :locked_at, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:google_oauth2_users, [:locale], where: "locale IS NULL")
    create index(:google_oauth2_users, [:hosted_domain], where: "hosted_domain IS NULL")
    create index(:google_oauth2_users, [:name], where: "name IS NULL")
    create index(:google_oauth2_users, [:given_name], where: "given_name IS NULL")
    create index(:google_oauth2_users, [:family_name], where: "family_name IS NULL")
    create index(:google_oauth2_users, [:gender], where: "gender IS NULL")
    create index(:google_oauth2_users, [:granted_scopes], using: :gin)
    create unique_index(:google_oauth2_users, [:client_id, :sub])
    create unique_index(:google_oauth2_users, [:email])
    create unique_index(:google_oauth2_users, [:sn])
  end
end
