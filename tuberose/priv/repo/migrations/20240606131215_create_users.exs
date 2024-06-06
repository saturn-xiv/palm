defmodule Tuberose.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users) do
      add(:uid, :string, size: 36, null: false)
      add(:name, :string, size: 31)
      add(:avatar, :string, size: 127)
      add(:lang, :string, size: 15, null: false, default: "en-US")
      add(:timezone, :string, size: 31, null: false, default: "UTC")
      add(:sign_in_count, :integer, default: 0, null: false)
      add(:current_sign_in_at, :utc_datetime)
      add(:current_sign_in_ip, :string, size: 45)
      add(:last_sign_in_at, :utc_datetime)
      add(:last_sign_in_ip, :string, size: 45)
      add(:locked_at, :utc_datetime)
      add(:deleted_at, :utc_datetime)
      add(:version, :integer, default: 0, null: false)

      timestamps(type: :utc_datetime)
    end

    create(unique_index(:users, [:uid]))
    create(index(:users, [:lang]))
    create(index(:users, [:timezone]))
    create(index(:users, [:name], where: "name IS NOT NULL"))
    create(index(:users, [:current_sign_in_ip], where: "current_sign_in_ip IS NOT NULL"))
    create(index(:users, [:last_sign_in_ip], where: "last_sign_in_ip IS NOT NULL"))
  end
end
