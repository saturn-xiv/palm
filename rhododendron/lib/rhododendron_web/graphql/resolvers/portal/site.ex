defmodule RhododendronWeb.Resolvers.Portal do
  def version(_parent, _args, _resolution) do
    {:ok,
     %{
       api: Application.spec(:rhododendron, :vsn) |> to_string,
       iana: Tzdata.tzdata_version()
     }}
  end

  def timezones(_parent, _args, _resolution) do
    {
      :ok,
      Tzdata.zone_alias_list()
    }
  end

  def currencies(_parent, _args, _resolution) do
    items =
      Enum.map(Rhododendron.Dao.Currency.index(), fn it ->
        %{
          id: it.id,
          name: it.name,
          code: it.code,
          country: it.country,
          number: it.number,
          units: it.units
        }
      end)

    {:ok, items}
  end
end
