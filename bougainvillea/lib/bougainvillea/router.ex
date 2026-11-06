defmodule Bougainvillea.Router do
  use Plug.Router
  use Plug.ErrorHandler

  plug(Plug.Logger)

  plug(:match)

  plug(Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json, Absinthe.Plug.Parser],
    pass: ["*/*"],
    json_decoder: Jason
  )

  plug(Plug.Parsers,
    parsers: [:urlencoded, :multipart],
    pass: ["*/*"],
    length: 1_000_000_000
  )

  plug(Bougainvillea.Graphql.Context)

  plug(Plug.Static,
    at: "/assets",
    from: {:bougainvillea, "priv/public/statics"}
  )

  plug(Plug.Static,
    at: "/3rd",
    from: {:bougainvillea, "priv/public/vendors"}
  )

  plug(:dispatch)

  forward("/graphql", to: Absinthe.Plug, init_opts: [schema: Bougainvillea.Graphql.Schema])

  forward("/graphiql",
    to: Absinthe.Plug.GraphiQL,
    init_opts: [schema: Bougainvillea.Graphql.Schema, interface: :simple]
  )

  get "/api/health-check" do
    {:ok, body} = Bougainvillea.Portal.Controllers.Api.health_check()
    Hibiscus.Render.json(conn, 200, body)
  end

  get "/attachments/:sn" do
    Bougainvillea.Portal.Controllers.Attachments.show(conn, sn)
  end

  post "/attachments" do
    {:ok, body} = Bougainvillea.Portal.Controllers.Attachments.create(conn)
    Hibiscus.Render.json(conn, 200, body)
  end

  get "/" do
    {:ok, body} = Bougainvillea.Portal.Controllers.Home.index()
    Hibiscus.Render.html(conn, 200, body)
  end

  match _ do
    Hibiscus.Render.text(conn, 404)
  end

  @impl Plug.ErrorHandler
  def handle_errors(conn, %{kind: _kind, reason: reason, stack: _stack}) do
    Hibiscus.Render.text(conn, conn.status, reason)
  end
end
