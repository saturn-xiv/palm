const std = @import("std");

pub const third = @cImport({
    @cInclude("third.h");
});

pub const Error = error{ PasswordSumFailed, PasswordNotMatch, EncryptFailed, InvalidEncryptedMessage };

pub fn encrypt(allocator: std.mem.Allocator, key: []const u8, plain: []const u8) !struct { cipher: []const u8, nonce: []const u8 } {
    const nonce = try allocator.alloc(u8, third.crypto_secretbox_NONCEBYTES);
    errdefer allocator.free(nonce);
    third.randombytes_buf(nonce.ptr, third.crypto_secretbox_NONCEBYTES);

    const buf = try allocator.alloc(u8, third.crypto_secretbox_MACBYTES + plain.len);
    errdefer allocator.free(buf);
    if (third.crypto_secretbox_easy(buf.ptr, plain.ptr, plain.len, nonce.ptr, key.ptr) != 0) {
        return Error.EncryptFailed;
    }

    return .{ .cipher = buf, .nonce = nonce };
}

pub fn decrypt(allocator: std.mem.Allocator, key: []const u8, cipher: []const u8, nonce: []const u8) ![]const u8 {
    if (cipher.len <= third.crypto_secretbox_MACBYTES) {
        return Error.InvalidEncryptedMessage;
    }
    const buf = try allocator.alloc(u8, cipher.len - third.crypto_secretbox_MACBYTES);
    errdefer allocator.free(buf);

    if (third.crypto_secretbox_open_easy(buf.ptr, cipher.ptr, cipher.len, nonce.ptr, key.ptr) != 0) {
        return Error.InvalidEncryptedMessage;
    }
    return buf;
}

pub fn hash(allocator: std.mem.Allocator, plain: []const u8) ![]const u8 {
    const buf = try allocator.alloc(u8, third.crypto_pwhash_STRBYTES);
    errdefer allocator.free(buf);
    if (third.crypto_pwhash_str(buf.ptr, plain.ptr, plain.len, third.crypto_pwhash_OPSLIMIT_SENSITIVE, third.crypto_pwhash_MEMLIMIT_SENSITIVE) != 0) {
        return Error.PasswordSumFailed;
    }
    return buf;
}

pub fn verify(cipher: []const u8, plain: []const u8) !void {
    if (third.crypto_pwhash_str_verify(cipher.ptr, plain.ptr, plain.len) != 0) {
        return Error.PasswordNotMatch;
    }
}

pub fn random_u32() u32 {
    return third.randombytes_random();
}
pub fn random_bytes(allocator: std.mem.Allocator, len: usize) ![]const u8 {
    const buf = try allocator.alloc(u8, len);
    third.randombytes_buf(buf.ptr, len);
    return buf;
}

test "sodium" {
    const allocator = std.testing.allocator;
    try std.testing.expect(third.sodium_init() >= 0);
    {
        const it = random_u32();
        {
            const tmp = random_u32();
            std.debug.print("Random u32: {d} vs {d}\n", .{ it, tmp });
            try std.testing.expect(it != tmp);
        }
    }

    {
        const len: usize = 16;
        const it = try random_bytes(allocator, len);
        defer allocator.free(it);
        try std.testing.expectEqual(it.len, len);
        {
            const tmp = try random_bytes(allocator, len);
            defer allocator.free(tmp);
            std.debug.print("Random bytes: {x} vs {x}\n", .{ it, tmp });
            try std.testing.expect(!std.mem.eql(u8, tmp, it));
        }
    }

    const hi = "Hello, Begonia!";
    {
        const it = try hash(allocator, hi);
        defer allocator.free(it);
        try std.testing.expectEqual(it.len, third.crypto_pwhash_STRBYTES);
        try verify(it, hi);
        {
            const tmp = try hash(allocator, hi);
            defer allocator.free(tmp);
            std.debug.print("Hashed: {s} vs {s}\n", .{ it, tmp });
            try std.testing.expect(!std.mem.eql(u8, tmp, it));
        }
        _ = verify(it, "hello, begonia!") catch {
            try std.testing.expect(true);
        };
    }

    {
        const key = try random_bytes(allocator, third.crypto_secretbox_KEYBYTES); // 32 bytes
        defer allocator.free(key);

        const it = try encrypt(allocator, key, hi);
        defer allocator.free(it.cipher);
        defer allocator.free(it.nonce);

        try std.testing.expectEqual(it.cipher.len, third.crypto_secretbox_MACBYTES + hi.len);

        {
            const tmp = try encrypt(allocator, key, hi);
            defer allocator.free(tmp.cipher);
            defer allocator.free(tmp.nonce);

            std.debug.print("Encrypted(cipher): {x} vs {x}\n", .{ it.cipher, tmp.cipher });
            std.debug.print("Encrypted(nonce): {x} vs {x}\n", .{ it.nonce, tmp.nonce });
            try std.testing.expect(!std.mem.eql(u8, tmp.cipher, it.cipher));
            try std.testing.expect(!std.mem.eql(u8, tmp.nonce, it.nonce));
        }
        {
            const tmp = try decrypt(allocator, key, it.cipher, it.nonce);
            defer allocator.free(tmp);
            try std.testing.expect(std.mem.eql(u8, tmp, hi));
        }
        {
            const new_key = try random_bytes(allocator, third.crypto_secretbox_KEYBYTES);
            defer allocator.free(new_key);
            _ = decrypt(allocator, new_key, it.cipher, it.nonce) catch {
                try std.testing.expect(true);
            };
        }
    }
}
