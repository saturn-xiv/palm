# Script for populating the database. You can run it as:
#
#     mix run priv/repo/seeds.exs
#
# Inside the script, you can read and write to any of your
# repositories directly:
#
#     Jasmine.Repo.insert!(%Jasmine.SomeSchema{})
#
# We recommend using the bang functions (`insert!`, `update!`
# and so on) as they will fail if something goes wrong.
require Logger
require Ecto.Query

defmodule(Locales) do
  def load(lang, file) do
    namespace = file |> Path.rootname() |> Path.basename()

    case Path.extname(file) do
      ".json" ->
        Logger.info("load locales(#{lang}, #{namespace}) from #{file}")
        file |> File.read!() |> Jason.decode!() |> parse(lang, namespace)

      _ ->
        Logger.warning("unsupported #{file}")
    end
  end

  def parse(doc, lng, ns) when is_map(doc) do
    Enum.each(doc, fn {k, v} ->
      k = "#{ns}.#{k}"

      if is_map(v) do
        parse(v, lng, k)
      else
        Logger.debug("found item #{lng}.#{k} => #{v}")

        %Jasmine.Locale{
          lang: lng,
          code: k,
          message: v,
          updated_at: DateTime.utc_now()
        }
        |> Jasmine.Repo.insert!()
      end
    end)
  end
end

load_locales = fn root ->
  count = Jasmine.Models.Locale.count()

  if count == 0 do
    Enum.each(File.ls!(root), fn node ->
      file = Path.join(root, node)

      if File.dir?(file) do
        Logger.info("find language #{node}")

        Enum.each(File.ls!(file), fn name ->
          file = Path.join(file, name)

          if File.regular?(file) do
            Locales.load(node, file)
          end
        end)
      end
    end)

    languages = Jasmine.Models.Locale.languages()

    Enum.each(File.ls!(root), fn node ->
      file = Path.join(root, node)

      if File.regular?(file) do
        Enum.each(languages, fn lang ->
          Locales.load(lang, file)
        end)
      end
    end)
  else
    Logger.info("found #{count} locale items")
  end
end

# https://www.iso.org/iso-4217-currency-codes.html
load_currencies = fn file ->
  count = Jasmine.Models.Currency.count()

  if count == 0 do
    Logger.info("load iso4217 from #{file}")
    {doc, _} = :xmerl_scan.file(file)
    items = :xmerl_xpath.string(~c'/ISO_4217/CcyTbl/CcyNtry', doc)

    Enum.each(items, fn node ->
      [{:xmlText, _, _, _, country, :text}] = :xmerl_xpath.string(~c'./CtryNm/text()', node)
      [{:xmlText, _, _, _, name, :text}] = :xmerl_xpath.string(~c'./CcyNm/text()', node)

      case :xmerl_xpath.string(~c'./Ccy/text()', node) do
        [{:xmlText, _, _, _, code, :text}] ->
          case :xmerl_xpath.string(~c'./CcyNbr/text()', node) do
            [{:xmlText, _, _, _, number, :text}] ->
              case :xmerl_xpath.string(~c'./CcyMnrUnts/text()', node) do
                [{:xmlText, _, _, _, units, :text}] ->
                  unless units === ~c'N.A.' do
                    Logger.info("found item #{name} #{code} #{country} #{number} #{units}")

                    %Jasmine.Currency{
                      name: name |> List.to_string(),
                      country: country |> List.to_string(),
                      code: code |> List.to_string(),
                      number: number |> List.to_string(),
                      units: units |> List.to_string() |> String.to_integer()
                    }
                    |> Jasmine.Repo.insert!()
                  end

                _ ->
                  nil
              end

            _ ->
              nil
          end

        _ ->
          nil
      end
    end)
  else
    Logger.info("found #{count} currencies items")
  end
end

Jasmine.Repo.transaction(fn ->
  load_locales.("priv/static/locales")
  load_currencies.("priv/static/iso4217/list-one.xml")
end)
