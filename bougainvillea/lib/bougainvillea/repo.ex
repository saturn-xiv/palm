defmodule Bougainvillea.Repo do
  use Ecto.Repo,
    otp_app: :bougainvillea,
    adapter: Ecto.Adapters.Postgres
end
