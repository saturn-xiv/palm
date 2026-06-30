const std = @import("std");
const c = @cImport({
    @cInclude("time.h");
});

const base64 = @import("base64.zig");

pub fn timestamp(allocator: std.mem.Allocator) ![]const u8 {
    var now: c.time_t = undefined;
    _ = c.time(&now);
    const info = c.localtime(&now);
    const buf = try allocator.alloc(u8, 15);
    _ = c.strftime(buf.ptr, buf.len, "%Y%m%d%H%M%S", info);
    return buf;
}

test "timestamp in seconds" {
    const allocator = std.testing.allocator;

    const buf = try timestamp(allocator);
    defer allocator.free(buf);

    std.debug.print("timestamp({}): {s}\n", .{ buf.len, buf });
}

pub const Random = struct {
    pub fn bytes(allocator: std.mem.Allocator, io: std.Io, len: usize) ![]const u8 {
        const buf = try allocator.alloc(u8, len);
        try std.Io.randomSecure(io, buf);
        return buf;
    }
    pub fn alphanumeric(allocator: std.mem.Allocator, io: std.Io, len: usize) ![]const u8 {
        const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        const rng_impl: std.Random.IoSource = .{ .io = io };
        const rng = rng_impl.interface();

        const buf = try allocator.alloc(u8, len);
        for (buf) |*it| {
            const i = rng.uintLessThan(usize, charset.len);
            it.* = charset[i];
        }
        return buf;
    }
};

test "random" {
    const allocator = std.testing.allocator;
    const io = std.testing.io;

    const len = 32;

    for (1..10) |i| {
        const buf = try Random.bytes(allocator, io, len);
        defer allocator.free(buf);

        try std.testing.expect(buf.len == len);
        const str = try base64.UrlSafeNoPadding.encode(allocator, buf);
        defer allocator.free(str);

        std.debug.print("random bytes({}): {s}\n", .{ i, str });
    }

    for (1..10) |i| {
        const buf = try Random.alphanumeric(allocator, io, len);
        defer allocator.free(buf);

        try std.testing.expect(buf.len == len);
        std.debug.print("random alphanumeric({}): {s}\n", .{ i, buf });
    }
}

// https://mad9scientist.com/dovecot-password-creation-php/
pub const Ssha512 = struct {
    pub fn sum(allocator: std.mem.Allocator, io: std.Io, password: []const u8, salt_len: usize) ![]const u8 {
        const salt = try Random.bytes(allocator, io, salt_len);
        defer allocator.free(salt);

        var digest: [std.crypto.hash.sha2.Sha512.digest_length]u8 = undefined;
        var hasher = std.crypto.hash.sha2.Sha512.init(.{});
        hasher.update(password);
        hasher.update(salt);
        hasher.final(&digest);

        const data = try std.mem.concat(allocator, u8, &.{ &digest, salt });
        defer allocator.free(data);
        return base64.StandardWithPadding.encode(allocator, data);
    }
    pub fn verify(allocator: std.mem.Allocator, hashed: []const u8, password: []const u8) !bool {
        const data = try base64.StandardWithPadding.decode(allocator, hashed);
        defer allocator.free(data);

        const salt = data[std.crypto.hash.sha2.Sha512.digest_length..];

        var digest: [std.crypto.hash.sha2.Sha512.digest_length]u8 = undefined;
        var hasher = std.crypto.hash.sha2.Sha512.init(.{});
        hasher.update(password);
        hasher.update(salt);
        hasher.final(&digest);

        return std.mem.eql(u8, &digest, data[0..std.crypto.hash.sha2.Sha512.digest_length]);
    }
};

test "ssha512" {
    const allocator = std.testing.allocator;
    const io = std.testing.io;

    const password = "Hi, Palm!";
    for (1..3) |i| {
        const hashed = try Ssha512.sum(allocator, io, password, 6);
        defer allocator.free(hashed);

        try std.testing.expect(try Ssha512.verify(allocator, hashed, password));
        std.debug.print("ssha512({}): doveadm pw -t '{s}{s}' -p '{s}'\n", .{ i, "{SSHA512}", hashed, password });
    }
}
