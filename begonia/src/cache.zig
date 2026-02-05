const std = @import("std");

pub const third = @cImport({
    @cInclude("third.h");
});

pub const Error = error{ ConnectionFailed, CommandFailed };

pub fn open(host: []const u8, port: u16) !*third.redisContext {
    const client = third.redis_open(host.ptr, port);
    if (client == null) {
        return Error.ConnectionFailed;
    }
    return client;
}

pub fn ping(client: *third.redisContext) !void {
    if (third.redis_ping(client) != 0) {
        return Error.CommandFailed;
    }
}

pub fn set(client: *third.redisContext, key: []const u8, value: []const u8, ttl: usize) !void {
    if (third.redis_set(client, key.ptr, value.ptr, value.len, ttl) != 0) {
        return Error.CommandFailed;
    }
}

pub fn get(allocator: std.mem.Allocator, client: *third.redisContext, key: []const u8, buffer_len: usize) ![]const u8 {
    // var buf: std.ArrayList(u8) = .empty;
    // try buf.ensureTotalCapacity(allocator, buffer_len);
    var buf = try std.ArrayList(u8).initCapacity(allocator, buffer_len);
    const len = third.redis_get(client, key.ptr, buf.items.ptr, buffer_len);
    if (len < 0) {
        return Error.CommandFailed;
    }
    try buf.resize(allocator, @as(usize, @intCast(len)));
    return buf.toOwnedSlice(allocator);
}

test "redis set-get" {
    const allocator = std.testing.allocator;

    const client = try open("127.0.0.1", 6379);
    try ping(client);
    {
        const key = "hi";
        const value = "Hello,begonia!";
        try set(client, key, value, @as(usize, @intCast(60 * 60 * 1)));
        {
            const tmp = try get(allocator, client, key, @as(usize, @intCast(1024)));
            defer allocator.free(tmp);
            try std.testing.expect(std.mem.eql(u8, tmp, value));
        }
    }
}
