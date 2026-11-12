defmodule Bougainvillea.Repo.Migrations.Portal.CreateAttachments do
  use Ecto.Migration

  def change do
    create table(:attachments) do
      add :user_id, references(:users, on_delete: :delete_all), null: false
      add :sn, :string, size: 127, null: false
      add :title, :string, size: 127, null: false
      add :size, :integer
      add :content_type, :string, size: 127, null: false
      add :public, :boolean, null: false
      add :uploaded, :utc_datetime_usec
      add :deleted_at, :utc_datetime_usec
      add :version, :integer, null: false, default: 0

      timestamps(type: :utc_datetime_usec)
    end

    create index(:attachments, [:title])
    create index(:attachments, [:content_type])
    create unique_index(:attachments, [:sn])
  end
end
