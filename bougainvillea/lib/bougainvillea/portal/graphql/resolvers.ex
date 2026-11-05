defmodule Bougainvillea.Portal.Graphql.Resolvers do
  def version(_parent, _args, _resolution) do
    {:ok, Application.spec(:bougainvillea, :vsn) |> to_string}
  end

  def succeed() do
    %{created_at: DateTime.utc_now(:microsecond)}
  end
end
