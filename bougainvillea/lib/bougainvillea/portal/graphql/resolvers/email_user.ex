defmodule Bougainvillea.Portal.Graphql.Resolvers.EmailUser do
  def sign_up(_parent, args, %{context: context}) do
    IO.inspect(args)
    IO.inspect(context)
    {:ok, Bougainvillea.Portal.Graphql.Resolvers.succeed()}
  end
end
