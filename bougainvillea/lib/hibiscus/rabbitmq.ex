defmodule Hibiscus.RabbitMq do
  require Logger
  @channel :bougainvillea

  def produce(queue, id, content_type, message) do
    Logger.info("send a #{content_type} message #{id}@#{queue}")
    {:ok, ch} = AMQP.Application.get_channel(@channel)

    AMQP.Queue.declare(ch, queue,
      durable: true,
      arguments: []
    )

    AMQP.Basic.publish(ch, "", queue, message, content_type: content_type, message_id: id)
  end
end
