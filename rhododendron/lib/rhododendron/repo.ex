defmodule Rhododendron.Repo do
  use Ecto.Repo,
    otp_app: :rhododendron,
    adapter: Ecto.Adapters.Postgres
end
