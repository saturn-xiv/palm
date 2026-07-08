defmodule Rhododendron.Dao.Currency do
  require Logger
  import Ecto.Query
  import SweetXml

  def load_from_one_xml(file) do
    Logger.info("load ISO4217 records from #{file}")
    {:ok, buf} = File.read(file)
    doc = parse(buf)

    result =
      doc
      |> xpath(
        ~x"//ISO_4217/CcyTbl/CcyNtry"l,
        country: ~x"./CtryNm/text()",
        name: ~x"./CcyNm/text()",
        code: ~x"./Ccy/text()",
        number: ~x"./CcyNbr/text()",
        units: ~x"./CcyMnrUnts/text()",
        is_fund: ~x"./CcyNm/@IsFund"
      )

    inserted =
      Enum.reduce(result, 0, fn %{
                                  name: name,
                                  code: code,
                                  country: country,
                                  number: number,
                                  units: units,
                                  is_fund: is_fund
                                },
                                acc ->
        Logger.debug("found (#{name}, #{code}, #{country}, #{number}, #{units}, #{is_fund})")

        if code != nil and number != nil and units != nil do
          units =
            if units == ~c"N.A.", do: nil, else: units |> List.to_string() |> String.to_integer()

          number = number |> List.to_string() |> String.to_integer()

          is_fund =
            if is_fund == nil do
              nil
            else
              if is_fund == ~c"true" do
                true
              else
                raise ArgumentError, message: "Invalid IsFund value"
              end
            end

          %Rhododendron.Currency{
            name: name |> List.to_string(),
            country: country |> List.to_string(),
            code: code |> List.to_string(),
            number: number,
            units: units,
            is_fund: is_fund
          }
          |> Rhododendron.Repo.insert!()

          acc + 1
        else
          acc
        end
      end)

    {:ok, %{total: length(result), inserted: inserted}}
  end

  def count() do
    Rhododendron.Repo.one!(from t in Rhododendron.Currency, select: count())
  end
end
