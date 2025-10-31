defmodule Mix.Tasks.Bamboo.ScanHosts do
  use Mix.Task

  require Logger
  require OptionParser
  require Validate
  import Ecto.Changeset
  import SweetXml

  @shortdoc "Setup an administrator account"
  def run(_args) do
    Mix.Task.run "app.start"

    # Bamboo.Repo.insert(%Bamboo.Setting{key: "network.interface.wlp0s20f3", value: :erlang.term_to_binary(%Bamboo.Network.Interface.Lan{name: "lan", network: "192.168.11.0"})})
    # Bamboo.Repo.insert(%Bamboo.Setting{key: "network.interface.enp5s0", value: :erlang.term_to_binary(%Bamboo.Network.Interface.Lan{name: "dmz", network: "192.168.12.0"})})

    Enum.each(Bamboo.Network.Interface.devices(), fn x ->
      scan_hosts x
    end)

  end

  def scan_hosts(id) do
    case Bamboo.Repo.get_by(Bamboo.Setting, key: "network.interface.#{id}") do
      nil ->
        Logger.warning("skip for scan network #{id}")
      it ->
        case :erlang.binary_to_term(it.value) do
          %Bamboo.Network.Interface.Lan{name: name, network: network, cidr: cidr} ->
            scan_hosts name, network, cidr
          _ ->
            Logger.warning("not an local network")
        end
    end
  end

  def scan_hosts(name, network, cidr) do
    tmp = Path.join("/tmp", "out-#{UUID.uuid4()}.xml")
    Logger.info("scan network #{name} into file #{tmp}")
    {out, 0} = System.cmd("nmap", ["-oX", tmp, "-sn", "#{network}/#{cidr}"])
    Logger.debug(out)
    {:ok, doc} = File.read(tmp)

    # sudo nmap -oX /tmp/aaa.xml -sn 192.168.12.0/24 192.168.11.0/24
    # {:ok, doc} = File.read("/tmp/aaa.xml")

    items = doc |> xpath(
      ~x"//host"l,
      address: ~x"./address/@addr"l,
      address_type: ~x"./address/@addrtype"l,
      hostname: ~x"./hostnames/hostname/@name"l,
      hostname_type: ~x"./hostnames/hostname/@type"l
    )
    Enum.each(items, fn x ->
      # Logger.debug("#{inspect(x)}")
      name = case List.first(x.hostname) do
        nil -> nil
        v -> to_string(v)
      end
      mac = case Enum.find_index(x.address_type, fn y -> y == ~c"mac" end) do
        nil ->
          nil
        i ->
          {:ok, v} = Enum.fetch(x.address, i)
          to_string(v)
      end
      ip = case Enum.find_index(x.address_type, fn y -> y == ~c"ipv4" end) do
        nil ->
          nil
        i ->
          {:ok, v} = Enum.fetch(x.address, i)
          to_string(v)
      end

      if not is_nil(mac) and not is_nil(ip) do
        case Bamboo.Repo.get_by(Bamboo.Host, mac: mac) do
          nil ->
            Logger.info("create host #{mac}")
            Bamboo.Repo.insert(%Bamboo.Host{mac: mac, ip: ip, name: name})
          it ->
            Logger.info("update host #{mac}")
            Bamboo.Repo.update(change(it, %{mac: mac, ip: ip, name: name}))
        end
      end

    end)

  end

end
