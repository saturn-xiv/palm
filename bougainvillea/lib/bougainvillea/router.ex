defmodule Bougainvillea.Router do
  use Plug.Router

  plug(:match)
  plug(:dispatch)

  get "/api/health-check" do
    body = Bougainvillea.Controllers.Api.health_check()
    Hibiscus.Render.json(conn, 200, body)
  end

  match _ do
    send_resp(conn, 404, "Page Not Found")
  end
end
