defmodule Bougainvillea.Hydrangea.Models.Job do
  use Ecto.Schema

  schema "hydrangea_jobs" do
    field :sn, :integer
    field :name, :string
    field :summary, :string
    field :version, :string
    field :args, :map
    field :script, :string
    field :launched_at, :utc_datetime_usec
    field :elapsed, :string
    field :status_code, :integer
    field :stdout, :string
    field :stderr, :string
    field :created_at, :utc_datetime_usec

    belongs_to :user, Bougainvillea.Portal.Models.User
  end
end
