const std = @import("std");

pub const Error = error{ InvalidToken, InvalidSignature, TimeExpired, InvalidIssuer, InvalidAudience };

const Header = struct {
    alg: []const u8,
    typ: []const u8,
};

const Payload = struct {
    iss: []const u8,
    sub: []const u8,
    aud: []const u8,
    iat: i64,
    exp: i64,
    nbf: i64,
};

// https://www.jwt.io/introduction#what-is-json-web-token-structure
// https://datatracker.ietf.org/doc/html/rfc7519#section-4.1
pub fn sum(allocator: std.mem.Allocator, key: []const u8, issuer: []const u8, subject: []const u8, audiences: []const []const u8, ttl: usize) ![]const u8 {
    const header = Header{ .alg = "HS512", .typ = "JWT" };
    const header_j = try std.json.Stringify.valueAlloc(allocator, header, .{});
    defer allocator.free(header_j);
    const header_s = try base64_encode(allocator, header_j);
    defer allocator.free(header_s);

    const audience = try std.json.Stringify.valueAlloc(allocator, audiences, .{});
    defer allocator.free(audience);
    const now = std.time.timestamp();
    const payload = Payload{
        .sub = subject,
        .iss = issuer,
        .aud = audience,
        .nbf = now - 1,
        .iat = now,
        .exp = now + @as(i64, @intCast(ttl)),
    };
    const payload_j = try std.json.Stringify.valueAlloc(allocator, payload, .{});
    defer allocator.free(payload_j);
    const payload_s = try base64_encode(allocator, payload_j);
    defer allocator.free(payload_s);

    var token: std.ArrayList(u8) = .empty;
    try token.appendSlice(allocator, header_s);
    try token.appendSlice(allocator, ".");
    try token.appendSlice(allocator, payload_s);

    const signature = try hmac_sign(allocator, key, token.items);
    defer allocator.free(signature);
    try token.appendSlice(allocator, ".");
    try token.appendSlice(allocator, signature);

    return token.toOwnedSlice(allocator);
}

pub fn verify(allocator: std.mem.Allocator, key: []const u8, token: []const u8, issuer: []const u8, audience: []const u8) ![]u8 {
    var items = std.mem.splitSequence(u8, token, ".");

    const header = items.next();
    if (header == null) {
        return Error.InvalidToken;
    }
    const payload = items.next();
    if (payload == null) {
        return Error.InvalidToken;
    }
    const signature = items.next();
    if (signature == null) {
        return Error.InvalidToken;
    }
    if (items.next() != null) {
        return Error.InvalidToken;
    }
    {
        var buf: std.ArrayList(u8) = .empty;
        defer buf.deinit(allocator);
        try buf.appendSlice(allocator, header.?);
        try buf.appendSlice(allocator, ".");
        try buf.appendSlice(allocator, payload.?);

        const tmp = try hmac_sign(allocator, key, buf.items);
        defer allocator.free(tmp);
        if (!std.mem.eql(u8, tmp, signature.?)) {
            return Error.InvalidSignature;
        }
    }

    const payload_j = try base64_decode(allocator, payload.?);
    defer allocator.free(payload_j);
    const payload_ = try std.json.parseFromSlice(Payload, allocator, payload_j, .{});
    defer payload_.deinit();

    const now = std.time.timestamp();
    if (payload_.value.nbf > now or payload_.value.exp < now) {
        return Error.TimeExpired;
    }

    if (!std.mem.eql(u8, payload_.value.iss, issuer)) {
        return Error.InvalidIssuer;
    }

    {
        const audiences = try std.json.parseFromSlice([]const []const u8, allocator, payload_.value.aud, .{});
        defer audiences.deinit();
        if (!in_audiences(audiences.value, audience)) {
            return Error.InvalidAudience;
        }
    }

    return try allocator.dupe(u8, payload_.value.sub);
}

pub const AUTHORIZATION = "Authorization";
pub const BEARER = "Bearer ";

fn in_audiences(items: []const []const u8, it: []const u8) bool {
    for (items) |el| {
        if (std.mem.eql(u8, it, el)) {
            return true;
        }
    }
    return false;
}

fn base64_decode(allocator: std.mem.Allocator, cipher: []const u8) ![]const u8 {
    const buf_len = try std.base64.url_safe_no_pad.Decoder.calcSizeForSlice(cipher);
    const buf = try allocator.alloc(u8, buf_len);
    try std.base64.url_safe_no_pad.Decoder.decode(buf, cipher);
    return buf[0..];
}

fn base64_encode(allocator: std.mem.Allocator, plain: []const u8) ![]const u8 {
    const buf_len = std.base64.url_safe_no_pad.Encoder.calcSize(plain.len);
    const buf = try allocator.alloc(u8, buf_len);
    return std.base64.url_safe_no_pad.Encoder.encode(buf, plain);
}

fn hmac_sign(allocator: std.mem.Allocator, key: []const u8, plain: []const u8) ![]const u8 {
    var auth = [_]u8{0} ** std.crypto.auth.hmac.sha2.HmacSha512.mac_length;
    std.crypto.auth.hmac.sha2.HmacSha512.create(&auth, plain, key);
    return base64_encode(allocator, &auth);
}

test "jwt" {
    const allocator = std.testing.allocator;
    const key = "QdPbpNRhlFgXb6DNLO5LYO4zYpkxy7jSl55hdtoSJSA=";
    const subject = "sss";
    const issuer = "iii";
    const audience = "aaa";

    {
        const audiences = &[_][]const u8{ "a0", "a1", audience, "a3" };
        const token = try sum(allocator, key, issuer, subject, audiences, 60);
        defer allocator.free(token);
        std.debug.print("JWT Token: {s}\n", .{token});

        {
            const tmp = try verify(allocator, key, token, issuer, audience);
            defer allocator.free(tmp);
            try std.testing.expect(std.mem.eql(u8, tmp, subject));
        }

        _ = verify(allocator, "Id3e9mZU7pDnnam4jl2Zl73oxcuzGqWGdDrJ9mHiQjc=", token, issuer, audience) catch {
            try std.testing.expect(true);
        };
        _ = verify(allocator, key, token, "in", audience) catch {
            try std.testing.expect(true);
        };
        _ = verify(allocator, key, token, issuer, "an") catch {
            try std.testing.expect(true);
        };
    }
}
