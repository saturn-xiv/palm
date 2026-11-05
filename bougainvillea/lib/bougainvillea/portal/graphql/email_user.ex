defmodule Bougainvillea.Graphql.Resolvers.Portal.EmailUser do
  def sign_up(_parent, args, _resolution) do
    IO.inspect(args)
    {:ok, Bougainvillea.Graphql.Resolvers.Portal.succeed()}
  end
end
