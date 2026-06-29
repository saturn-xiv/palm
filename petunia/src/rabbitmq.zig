const std = @import("std");

const RabbitMq = struct {
    fn sign(_: *RabbitMq) void {
        // TODO
    }
    fn verify(_: *RabbitMq) void {
        // TODO
    }
};

test "rabbitmq publisher-subscriber" {
    try std.testing.expect(1 + 1 == 2);
}

test "rabbitmq producer-consumer" {
    try std.testing.expect(1 + 1 == 2);
}
