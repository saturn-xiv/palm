const std = @import("std");

pub const Error = error{ Ssha512InvalidPrefix, Ssha512InvalidLength, Ssha512NotMatched };

pub const third = @cImport({
    @cInclude("third.h");
});

// https://mad9scientist.com/dovecot-password-creation-php/
pub fn ssha512_verify(allocator: std.mem.Allocator, cipher: []const u8, plain: []const u8) !void {
    if (!std.mem.startsWith(u8, cipher, SSHA512_PREFIX)) {
        return Error.Ssha512InvalidPrefix;
    }
    const buf_len = try std.base64.standard.Decoder.calcSizeForSlice(cipher[SSHA512_PREFIX.len..]);
    if (buf_len <= std.crypto.hash.sha2.Sha512.digest_length) {
        return Error.Ssha512InvalidLength;
    }
    const buf = try allocator.alloc(u8, buf_len);
    defer allocator.free(buf);

    try std.base64.standard.Decoder.decode(buf, cipher[SSHA512_PREFIX.len..]);

    const tmp = try _ssha512_sum(allocator, plain, buf[std.crypto.hash.sha2.Sha512.digest_length..]);
    defer allocator.free(tmp);

    if (!std.mem.eql(u8, tmp, buf)) {
        return Error.Ssha512NotMatched;
    }
}
pub fn ssha512_sum(allocator: std.mem.Allocator, plain: []const u8, salt_len: usize) ![]const u8 {
    const salt = try allocator.alloc(u8, salt_len);
    defer allocator.free(salt);
    std.crypto.random.bytes(salt);

    const buf = try _ssha512_sum(allocator, plain, salt);
    defer allocator.free(buf);

    const tmp_len = std.base64.standard.Encoder.calcSize(buf.len);
    const tmp = try allocator.alloc(u8, tmp_len);
    defer allocator.free(tmp);
    return try std.mem.concat(allocator, u8, &.{ SSHA512_PREFIX, std.base64.standard.Encoder.encode(tmp, buf) });
}

fn _ssha512_sum(allocator: std.mem.Allocator, plain: []const u8, salt: []const u8) ![]const u8 {
    var hasher = std.crypto.hash.sha2.Sha512.init(.{});
    hasher.update(plain);
    hasher.update(salt);
    var hash: [std.crypto.hash.sha2.Sha512.digest_length]u8 = undefined;
    hasher.final(&hash);

    var buf: std.ArrayList(u8) = .empty;
    try buf.appendSlice(allocator, &hash);
    try buf.appendSlice(allocator, salt);
    return buf.toOwnedSlice(allocator);
}

const SSHA512_PREFIX = "{SSHA512}";

test "jwt" {}
test "aead" {}
test "hmac" {}

test "ssha512" {
    const allocator = std.testing.allocator;
    const plain = "Hi, Begonia!";
    const cipher = try ssha512_sum(allocator, plain, 16);
    defer allocator.free(cipher);
    std.debug.print("doveadm pw -t '{s}' -p '{s}'\n", .{ cipher, plain });
    try ssha512_verify(allocator, cipher, plain);
    {
        const tmp = try ssha512_sum(allocator, plain, 16);
        defer allocator.free(tmp);
        try ssha512_verify(allocator, tmp, plain);
        try std.testing.expect(!std.mem.eql(u8, tmp, cipher));
    }

    ssha512_verify(allocator, cipher, "Hi, begonia!") catch {
        try std.testing.expect(true);
    };
}
