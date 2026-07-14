defmodule Rhododendron.Dao.Locale do
  require Logger
  import Ecto.Query

  def by_lang(lang) do
    Rhododendron.Repo.all(
      from t in Rhododendron.Locale, where: t.lang == ^lang, order_by: [asc: t.code]
    )
  end

  def languages() do
    Rhododendron.Repo.all(
      from t in Rhododendron.Locale, distinct: true, select: t.lang, order_by: [asc: t.lang]
    )
  end

  def load_from_yml(folder) do
    Logger.info("load locale files from #{folder}")

    result =
      Enum.reduce(File.ls!(folder), %{total: 0, inserted: 0}, fn file,
                                                                 %{
                                                                   total: total,
                                                                   inserted: inserted
                                                                 } ->
        dir = Path.join(folder, file)

        if File.dir?(dir) do
          %{total: t, inserted: i} = load_from_yml_folder(file, dir)
          %{total: total + t, inserted: inserted + i}
        else
          %{total: total, inserted: inserted}
        end
      end)

    {:ok, result}
  end

  defp load_from_yml_folder(lang, folder) do
    Logger.debug("Load from #{folder} for language #{lang}")

    Enum.reduce(File.ls!(folder), %{total: 0, inserted: 0}, fn file,
                                                               %{
                                                                 total: total,
                                                                 inserted: inserted
                                                               } ->
      dir = Path.join(folder, file)

      if File.regular?(dir) and Path.extname(dir) == ".yml" do
        %{total: t, inserted: i} = load_from_yml_file(lang, Path.rootname(file), dir)
        %{total: total + t, inserted: inserted + i}
      else
        %{total: total, inserted: inserted}
      end
    end)
  end

  defp load_from_yml_file(lang, code, file) do
    Logger.debug("Load from #{file} for #{lang}.#{code}")
    yml = YamlElixir.read_from_file!(file)
    load_yml_item(lang, nil, code, yml)
  end

  defp load_yml_item(lang, zone, key, value) when is_binary(value) do
    code = "#{zone}.#{key}"
    Logger.debug("Found #{lang}.#{code}")

    if Rhododendron.Repo.one!(
         from t in Rhododendron.Locale,
           select: count(),
           where: t.lang == ^lang and t.code == ^code
       ) == 0 do
      %Rhododendron.Locale{lang: lang, code: code, message: value} |> Rhododendron.Repo.insert!()
      %{total: 1, inserted: 1}
    else
      %{total: 1, inserted: 0}
    end
  end

  defp load_yml_item(lang, zone, key, value) when is_map(value) do
    Enum.reduce(value, %{total: 0, inserted: 0}, fn {k, v},
                                                    %{
                                                      total: total,
                                                      inserted: inserted
                                                    } ->
      zone = if zone == nil, do: key, else: "#{zone}.#{key}"
      %{total: t, inserted: i} = load_yml_item(lang, zone, k, v)
      %{total: total + t, inserted: inserted + i}
    end)
  end
end
