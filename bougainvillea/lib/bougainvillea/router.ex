defmodule Bougainvillea.Router do
  use Plug.Router

  plug(:match)
  plug(:dispatch)

  get "/hi" do
    send_resp(conn, 200, "Hello, Bougainvillea!")
  end

  match _ do
    send_resp(conn, 404, "Page Not Found")
  end
end
