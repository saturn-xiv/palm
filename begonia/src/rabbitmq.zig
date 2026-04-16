const std = @import("std");
pub const third = @cImport({
    @cInclude("third.h");
});

pub const Error = error{
    CreatingTcpSocket,
    OpeningTcpSocket,
    LoginFailed,
    OpeningChannel,
    QueueDeclareFailed,
};
pub fn open(
    host: []const u8,
    port: u16,
    user: []const u8,
    password: []const u8,
    virtual_host: []const u8,
    channel_id: u16,
) !third.amqp_connection_state_t {
    const logger = third.log4c_category_get("rabbitmq");

    const connection = third.amqp_new_connection();
    const socket = third.amqp_tcp_socket_new(connection);
    if (socket == null) {
        return Error.CreatingTcpSocket;
    }
    third.debug(logger, "open rabbitmq://%$s@%$s:%d/%s", user.ptr, host.ptr, port, virtual_host.ptr);

    if (third.amqp_socket_open(socket, host.ptr, port) != third.AMQP_STATUS_OK) {
        return Error.OpeningTcpSocket;
    }

    {
        const reply = third.amqp_login(
            connection,
            virtual_host.ptr,
            0,
            (1 << 10) * 128,
            0,
            third.AMQP_SASL_METHOD_PLAIN,
            user.ptr,
            password.ptr,
        );
        if (reply.reply_type != third.AMQP_RESPONSE_NORMAL) {
            return Error.LoginFailed;
        }
    }
    {
        _ = third.amqp_channel_open(connection, channel_id);
        const reply = third.amqp_get_rpc_reply(connection);
        if (reply.reply_type != third.AMQP_RESPONSE_NORMAL) {
            return Error.OpeningChannel;
        }
    }
    return connection;
}

pub fn ping(connection: third.amqp_connection_state_t) !void {
    const max = third.amqp_get_channel_max(connection);
    const logger = third.log4c_category_get("rabbitmq");
    third.debug(logger, "max %d channels", max);
}

pub fn declare_anonymous_queue(
    allocator: std.mem.Allocator,
    connection: third.amqp_connection_state_t,
    channel_id: u16,
    durable: bool,
    exclusive: bool,
    auto_delete: bool,
) ![]const u8 {
    const tmp = third.amqp_queue_declare(
        connection,
        channel_id,
        third.amqp_empty_bytes,
        0,
        if (durable) 1 else 0,
        if (exclusive) 1 else 0,
        if (auto_delete) 1 else 0,
        third.amqp_empty_table,
    );
    {
        const reply = third.amqp_get_rpc_reply(connection);
        if (reply.reply_type != third.AMQP_RESPONSE_NORMAL) {
            return Error.QueueDeclareFailed;
        }
    }

    const buf = try allocator.alloc(u8, tmp.queue.len);
    @memcpy(buf, tmp.queue.bytes[0..tmp.queue.len]);
    return buf;
}

// rabbitmqctl add_user www "change-me"
// rabbitmqctl add_vhost begonia-testing;
// rabbitmqctl set_permissions -p begonia-testing www ".*" ".*" ".*"
test "rabbitmq" {
    const channel_id: u16 = 1;
    const allocator = std.testing.allocator;
    const con = try open("127.0.0.1", 5672, "www", "change-me", "begonia-testing", channel_id);
    try ping(con);

    {
        const queue = try declare_anonymous_queue(allocator, con, channel_id, true, true, true);
        defer allocator.free(queue);
        std.debug.print("create an anonymous queue {s}", .{queue});
    }
}
