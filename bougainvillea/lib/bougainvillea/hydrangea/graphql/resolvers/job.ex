defmodule Bougainvillea.Hydrangea.Graphql.Resolvers.Job do
  def execute(_parent, %{sn: sn, args: args}, %{token: token, client_ip: client_ip}) do
    # TODO

    {:ok, Bougainvillea.Portal.Graphql.Resolvers.succeed()}
  end

  def index(_parent, %{page: page}, %{token: token}) do
    # TODO

    {:ok, Bougainvillea.Portal.Graphql.Resolvers.succeed()}
  end
end
