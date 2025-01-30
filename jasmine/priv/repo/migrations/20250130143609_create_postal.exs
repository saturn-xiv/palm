defmodule Jasmine.Repo.Migrations.CreatePostal do
  use Ecto.Migration

  def up do
    create table(:postal_recipients) do
      add :name, :string, size: 63, null: false
      add :phone, :string, size: 31
      add :fax, :string, size: 31
      add :email, :string, size: 31
      add :whatsapp, :string, size: 31
      add :wechat, :string, size: 31
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:postal_recipients, [:name])
    create index(:postal_recipients, [:phone], where: "phone IS NOT NULL")
    create index(:postal_recipients, [:fax], where: "fax IS NOT NULL")
    create index(:postal_recipients, [:email], where: "email IS NOT NULL")
    create index(:postal_recipients, [:whatsapp], where: "whatsapp IS NOT NULL")
    create index(:postal_recipients, [:wechat], where: "wechat IS NOT NULL")

    create table(:postal_addresses) do
      add :unit, :string, size: 7
      add :building, :string, size: 31
      add :street, :string, size: 127, null: false
      add :city, :string, size: 63, null: false
      add :province, :string, size: 63, null: false
      add :country, :string, size: 63, null: false
      add :zip_code, :string, size: 15, null: false
      add :passcode, :string, size: 15
      add :google_map, :string, size: 255
      add :a_map, :string, size: 255
      add :deleted_at, :utc_datetime
      add :version, :integer, null: false, default: 0
      add :updated_at, :utc_datetime, null: false
      add :created_at, :utc_datetime, null: false, default: fragment("CURRENT_TIMESTAMP")
    end

    create index(:postal_addresses, [:unit], where: "unit IS NOT NULL")
    create index(:postal_addresses, [:building], where: "building IS NOT NULL")
    create index(:postal_addresses, [:street])
    create index(:postal_addresses, [:city])
    create index(:postal_addresses, [:province])
    create index(:postal_addresses, [:country])
    create index(:postal_addresses, [:zip_code])
  end

  def down do
    drop table(:postal_addresses)
    drop table(:postal_recipients)
  end
end
