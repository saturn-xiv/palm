defmodule RhododendronWeb.Resolvers.Locale do
  def by_lang(_parent, %{lang: lang}, _resolution) do
    items =
      Enum.map(Rhododendron.Dao.Locale.by_lang(lang), fn it ->
        %{
          id: it.id,
          lang: it.lang,
          code: it.code,
          message: it.message,
          inserted_at: it.inserted_at
        }
      end)

    {:ok, items}
  end
end
