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

load_locales = fn ->
  count = Jasmine.Repo.one!(Ecto.Query.from(p in Jasmine.Locale, select: count()))

  if count == 0 do
    Logger.info("load locales from filesystem")
  else
    Logger.info("#{count} locale items")
  end
end

# https://www.iso.org/iso-4217-currency-codes.html
load_currencies = fn file ->
  count = Jasmine.Repo.one!(Ecto.Query.from(p in Jasmine.Currency, select: count()))

  if count == 0 do
    Logger.info("load iso4217 from #{file}")
    {doc, _} = :xmerl_scan.file(file)
    items = :xmerl_xpath.string(~c'/ISO_4217/CcyTbl/CcyNtry', doc)

    Enum.each(items, fn node ->
      # Logger.info("found item #{inspect(node)}")
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

      # [{:xmlText, _, _, _, code, :text}] = :xmerl_xpath.string(~c'./Ccy/text()', node)
      # [{:xmlText, _, _, _, number, :text}] = :xmerl_xpath.string(~c'./CcyNbr/text()', node)
      # [{:xmlText, _, _, _, units, :text}] = :xmerl_xpath.string(~c'./CcyMnrUnts/text()', node)
      # Logger.info("found item #{country}")
    end)
  else
    Logger.info("#{count} currencies items")
  end
end

load_locales.()
load_currencies.("priv/static/iso4217/list-one.xml")
