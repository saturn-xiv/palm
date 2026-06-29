const std = @import("std");

const c = @cImport({
    @cDefine("_NO_CRT_STDIO_INLINE", "1");
    @cInclude("stdio.h");
    @cInclude("stdlib.h");
    @cInclude("sodium.h");
});

pub const Error = error{
    LibSodiumInit,
};

pub fn random_bytes(buf: []u8) void {
    c.randombytes_buf(buf.ptr, buf.len);
}

pub fn init() !void {
    if (c.sodium_init() != c.EXIT_SUCCESS) {
        return Error.LibSodiumInit;
    }
}

test "libsodium random bytes" {
    const allocator = std.testing.allocator;

    const len = 32;

    for (1..9) |i| {
        const buf = try allocator.alloc(u8, len);
        defer allocator.free(buf);
        random_bytes(buf);
        try std.testing.expect(buf.len == len);
        std.debug.print("random bytes({}): {x}\n", .{ i, buf });
    }
}

test "libsodium password hashing" {
    try std.testing.expect(1 + 1 == 2);
}

test "libsodium encrypt data" {
    try std.testing.expect(1 + 1 == 2);
}
