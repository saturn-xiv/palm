defmodule Bougainvillea.Portal.Graphql.Resolvers.EmailUser do
  def sign_in(_parent, args, %{context: context}) do
    # TODO
    IO.inspect(args)
    IO.inspect(context)
    {:ok, Bougainvillea.Portal.Graphql.succeed()}
  end

  def sign_up(_parent, args, %{context: context}) do
    # TODO
    IO.inspect(args)
    IO.inspect(context)
    {:ok, Bougainvillea.Portal.Graphql.succeed()}
  end
end
