defmodule Tuberose.Repo do
  use Ecto.Repo,
    otp_app: :tuberose,
    adapter: Ecto.Adapters.Postgres
end
