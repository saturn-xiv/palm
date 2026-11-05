defmodule Bougainvillea.Graphql.Resolvers.Portal.EmailUser do
  def sign_up(_parent, args, %{context: context}) do
    IO.inspect(args)
    IO.inspect(context)
    {:ok, Bougainvillea.Graphql.Resolvers.Portal.succeed()}
  end
end
