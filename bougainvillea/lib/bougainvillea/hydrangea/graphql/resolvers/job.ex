defmodule Bougainvillea.Hydrangea.Graphql.Resolvers.Job do
  require Logger

  def execute(_parent, %{sn: sn, args: args}, %{token: token, client_ip: client_ip}) do
    {:ok, ss} = can(token)
    Logger.info("#{ss}@#{client_ip} execute background job #{sn} with #{Enum.join(args, ",")} ")

    {:ok, Bougainvillea.Portal.Graphql.succeed()}
  end

  def index(_parent, %{page: page}, %{token: token}) do
    {:ok, _} = can(token)
    total = Bougainvillea.Hydrangea.Dao.Job.count()
    pagination = Hibiscus.pagination(page.index, page.size, total)
    {offset, limit} = Hibiscus.page(pagination.index, pagination.size)
    items = Bougainvillea.Hydrangea.Dao.Job.index(offset, limit)

    {:ok,
     %{
       pagination: pagination,
       items:
         Enum.map(items, fn x ->
           %{
             id: x.id,
             sn: x.sn,
             name: x.name,
             summary: x.summary,
             version: x.version,
             script: x.script,
             args: x.args,
             launched_at: x.launched_at,
             elapsed: x.elapsed,
             status_code: x.status_code,
             stdout: x.stdout,
             stderr: x.stderr,
             created_at: x.created_at
           }
         end)
     }}
  end

  defp can(token) do
    {:ok, ss} = Bougainvillea.Portal.Graphql.current_user(token)

    if Bougainvillea.Portal.Graphql.Session.has?(ss, :bougainvillea) do
      {:ok, ss}
    else
      {:error, "Forbidden"}
    end
  end
end
