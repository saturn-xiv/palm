defmodule Bougainvillea.Hydrangea.Models.Job do
  use Ecto.Schema
  import Ecto.Changeset

  schema "hydrangea_jobs" do
    field :code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(hydrangea_job, attrs) do
    hydrangea_job
    |> cast(attrs, [:code])
    |> validate_required([:code])
  end
end
