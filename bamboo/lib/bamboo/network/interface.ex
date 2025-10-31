defmodule Bamboo.Network.Interface do
  def key(device) do
    "network.interface.#{device}"
  end

  def devices() do
    {:ok, items} = File.ls("/sys/class/net")
    Enum.reject(items, fn x -> x == "lo" end)
  end
end
