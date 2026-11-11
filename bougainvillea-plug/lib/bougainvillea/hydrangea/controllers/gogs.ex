defmodule Bougainvillea.Hydrangea.Controllers.Gogs do
  require Logger
  # https://gogs.io/advancing/webhooks
  def webhook(conn) do
    id = conn.params["id"]
    IO.inspect(conn.body_params)
    {:ok, delivery} = Bougainvillea.Portal.Controllers.header(conn, "X-Gogs-Delivery")
    {:ok, event} = Bougainvillea.Portal.Controllers.header(conn, "X-Gogs-Event")
    {:ok, signature} = Bougainvillea.Portal.Controllers.header(conn, "X-Gogs-Signature")
    {:ok, body, conn} = Plug.Conn.read_body(conn)
    Logger.info("receive Gogs message #{id}, #{delivery}, #{event}")
    Logger.debug("$$$ #{body}")
    {:ok, conn, "Hi, #{id}"}
  end
end
