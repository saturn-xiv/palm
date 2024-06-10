defmodule TuberoseWeb.Resolvers.LeaveWord do
  require Logger

  def create(_parent, %{content: content, editor: editor}, %{context: context}) do
    %Tuberose.LeaveWord{
      lang: context.locale,
      ip: context.client_ip,
      body: content,
      editor: editor,
      status: "pending"
    }
    |> Tuberose.Repo.insert()

    {:ok, %{created_at: DateTime.utc_now()}}
  end

  def index(_parent, %{pager: pager}, _resolution) do
    # TODO
    Logger.info("#{inspect(pager)}")

    {:ok,
     %{
       :pagination => %{
         :size => 5,
         :total => 10,
         :page => 2,
         :has_previous => false,
         :has_next => true
       },
       :items => []
     }}
  end
end
