const std = @import("std");

const c = @cImport({
    @cDefine("_NO_CRT_STDIO_INLINE", "1");
    @cInclude("stdio.h");
    @cInclude("stdlib.h");
    @cInclude("sodium.h");
});
const base64 = @import("base64.zig");

pub const Error = error{
    LibsodiumInit,
    LibsodiumPasswordSum,
    LibsodiumPasswordVerify,
    LibsodiumEncryptData,
    LibsodiumDecryptInvalidData,
    LibsodiumDecryptData,
};

pub const Random = struct {
    pub fn bytes(allocator: std.mem.Allocator, len: usize) ![]const u8 {
        const buf = try allocator.alloc(u8, len);
        c.randombytes_buf(buf.ptr, buf.len);
        return buf;
    }
};

pub const PasswordHashing = struct {
    pub fn sum(allocator: std.mem.Allocator, password: []const u8) ![]const u8 {
        const buf = try allocator.alloc(u8, c.crypto_pwhash_STRBYTES);
        if (c.crypto_pwhash_str(buf.ptr, password.ptr, password.len, c.crypto_pwhash_OPSLIMIT_SENSITIVE, c.crypto_pwhash_MEMLIMIT_SENSITIVE) != 0) {
            return Error.LibsodiumPasswordSum;
        }
        return buf;
    }
    pub fn verify(hashed: []const u8, password: []const u8) !void {
        if (c.crypto_pwhash_str_verify(hashed.ptr, password.ptr, password.len) != c.EXIT_SUCCESS) {
            return Error.LibsodiumPasswordVerify;
        }
    }
};

pub const SecretBox = struct {
    key: []const u8,
    pub fn init(key: []const u8) SecretBox {
        return SecretBox{
            .key = key,
        };
    }

    pub fn encrypt(self: SecretBox, allocator: std.mem.Allocator, plain: []const u8) !struct { []const u8, []const u8 } {
        const cipher = try allocator.alloc(u8, plain.len + c.crypto_secretbox_MACBYTES);
        const nonce = try allocator.alloc(u8, c.crypto_secretbox_NONCEBYTES);
        c.randombytes_buf(nonce.ptr, nonce.len);
        if (c.crypto_secretbox_easy(cipher.ptr, plain.ptr, plain.len, nonce.ptr, self.key.ptr) != c.EXIT_SUCCESS) {
            return Error.LibsodiumEncryptData;
        }
        return .{ cipher, nonce };
    }
    pub fn decrypt(self: SecretBox, allocator: std.mem.Allocator, cipher: []const u8, nonce: []const u8) ![]const u8 {
        if (cipher.len <= c.crypto_secretbox_MACBYTES) {
            return Error.LibsodiumDecryptInvalidData;
        }
        const plain = try allocator.alloc(u8, cipher.len - c.crypto_secretbox_MACBYTES);
        if (c.crypto_secretbox_open_easy(plain.ptr, cipher.ptr, cipher.len, nonce.ptr, self.key.ptr) != c.EXIT_SUCCESS) {
            return Error.LibsodiumDecryptData;
        }
        return plain;
    }
};

pub fn init() !void {
    if (c.sodium_init() < 0) {
        return Error.LibsodiumInit;
    }
}

test "libsodium random bytes" {
    try init();

    const allocator = std.testing.allocator;

    const len = 32;

    for (1..10) |i| {
        const buf = try Random.bytes(allocator, len);
        defer allocator.free(buf);

        try std.testing.expect(buf.len == len);
        std.debug.print("random bytes({}): {x}\n", .{ i, buf });
    }
}

test "libsodium password hashing" {
    try init();

    const allocator = std.testing.allocator;
    const password = "Hello, Palm!";
    for (1..3) |i| {
        const hashed = try PasswordHashing.sum(allocator, password);
        defer allocator.free(hashed);
        try std.testing.expect(hashed.len == c.crypto_pwhash_STRBYTES);

        {
            const str = try base64.encode(allocator, hashed);
            defer allocator.free(str);
            std.debug.print("hash password({}, {}): {s}\n", .{ i, hashed.len, str });
        }
    }
}

test "libsodium encrypt data" {
    try init();

    const allocator = std.testing.allocator;
    const plain = "Hello, Palm!";

    const key = try allocator.alloc(u8, c.crypto_secretbox_KEYBYTES);
    defer allocator.free(key);
    c.crypto_secretbox_keygen(key.ptr);
    std.debug.print("key({}), {x}\n", .{ c.crypto_secretbox_KEYBYTES, key });

    const secret_box = SecretBox.init(key);
    for (1..3) |i| {
        const cipher = try secret_box.encrypt(allocator, plain);
        defer allocator.free(cipher.@"0");
        defer allocator.free(cipher.@"1");

        {
            const str = try base64.encode(allocator, cipher.@"0");
            defer allocator.free(str);
            std.debug.print("encrypt({}, {}): {s}\n", .{ i, cipher.@"0".len, str });
        }
    }
}
