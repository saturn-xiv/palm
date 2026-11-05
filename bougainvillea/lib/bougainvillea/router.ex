defmodule Bougainvillea.Router do
  use Plug.Router

  plug(Plug.Logger)

  plug(:match)

  plug(Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json, Absinthe.Plug.Parser],
    pass: ["*/*"],
    json_decoder: Jason
  )

  plug(Bougainvillea.Graphql.Context)

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

  get "/" do
    {:ok, body} = Bougainvillea.Portal.Controllers.Html.home()
    Hibiscus.Render.html(conn, 200, body)
  end

  match _ do
    Hibiscus.Render.text(conn, 404)
  end
end
