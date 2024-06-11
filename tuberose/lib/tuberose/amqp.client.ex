defmodule Tuberose.Amqp.Client do
  require Logger

  def produce(queue, content_type, task) do
    send("", queue, content_type, task)
  end

  def send(exchange, routing_key, content_type, task) when content_type == :protobuf do
    message_id = Ecto.UUID.generate()

    Logger.info(
      "publish message(#{message_id}) to (#{exchange}, #{routing_key}) with content_type(#{content_type})"
    )

    {connection, channel} = open()

    case AMQP.Basic.publish(channel, exchange, routing_key, task,
           content_type: "application/grpc+proto",
           message_id: message_id,
           mandatory: true,
           persistent: true
         ) do
      :ok -> Logger.info("done")
      err -> Logger.error("#{err}")
    end

    AMQP.Connection.close(connection)
  end

  defp open() do
    {:ok, connection} =
      AMQP.Connection.open(Application.get_env(:tuberose, Tuberose.AmqpClient)[:url])

    {:ok, channel} = AMQP.Channel.open(connection)
    {connection, channel}
  end
end
