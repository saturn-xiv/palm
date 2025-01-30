defmodule Jasmine.Repo do
  use Ecto.Repo,
    otp_app: :jasmine,
    adapter: Ecto.Adapters.Postgres
end
