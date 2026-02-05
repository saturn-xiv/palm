const std = @import("std");
const third = @cImport({
    @cInclude("third.h");
});

pub const Error = error{ RegexNotMatch, InvalidLength };

pub fn email(allocator: std.mem.Allocator, original: []const u8) ![]const u8 {
    const it = try std.ascii.allocLowerString(allocator, trim(original));
    errdefer allocator.free(it);
    if (third.match(it.ptr, "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$") == 0) {
        return it;
    }
    return Error.RegexNotMatch;
}

pub fn range(str: []const u8, min_len: usize, max_len: usize) !void {
    if (str.len < min_len or str.len > max_len) {
        return Error.InvalidLength;
    }
}

pub fn alphanumeric(str: []const u8) !void {
    if (third.match(str.ptr, "^[[:alnum:]]+$") == 0) {
        return;
    }
    return Error.RegexNotMatch;
}

pub fn password(str: []const u8) !void {
    // FIXME usging regex
    // if (third.match(str.ptr, "^(?=.*\\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[!@#$%^&*()_+\\-=\\[\\]{};':\"\\\\|,.<>\\/?]).{8,16}$") == 0) {
    //     return;
    // }
    if (str.len >= 8 and str.len <= 31) {
        return;
    }
    return Error.RegexNotMatch;
}

pub fn code(allocator: std.mem.Allocator, original: []const u8) ![]const u8 {
    const it = try std.ascii.allocLowerString(allocator, trim(original));
    errdefer allocator.free(it);
    return it;
}

pub fn trim(original: []const u8) []const u8 {
    const whitespace = " \n\r\t";
    return std.mem.trim(u8, original, whitespace);
}

test "validate code" {
    const allocator = std.testing.allocator;
    const items = [_][]const u8{
        " Hello ☰  \t ",
        " Hello ☰ ",
        "Hello ☰ ",
    };
    for (items) |it| {
        const iv = try code(allocator, it);
        defer allocator.free(iv);
        std.debug.print("'{s}' => '{s}';\n", .{ it, iv });
        try std.testing.expect(std.mem.eql(u8, iv, "hello ☰"));
    }
}

test "validate email" {
    const allocator = std.testing.allocator;

    {
        const it = " Who-Am-I@Change-Me.com ";
        const iv = try email(allocator, it);
        defer allocator.free(iv);
        std.debug.print("'{s}' => '{s}';\n", .{ it, iv });
        try std.testing.expect(std.mem.eql(u8, iv, "who-am-i@change-me.com"));
    }

    {
        const it = " Who-Am-I@Ch☰ange-Me.com ";

        if (email(allocator, it)) |iv| {
            defer allocator.free(iv);
            try std.testing.expect(false);
        } else |_| {
            try std.testing.expect(true);
        }
    }
}

test "validate alphanumeric" {
    {
        const items = [_][]const u8{ "123", "1ac", "a2b", "abc" };
        for (items) |it| {
            try alphanumeric(it);
        }
    }

    {
        const items = [_][]const u8{ "a☰", " 1ac", "a2b ", "ab c" };
        for (items) |it| {
            alphanumeric(it) catch {
                try std.testing.expect(true);
            };
        }
    }
}

test "validate password" {
    {
        const items = [_][]const u8{ "StrongP@ss1", "-Secr3t.", "A1we&*eder.23", "1&*()%$#_=+/[]{}ac", "!@#$%^&*()_+{}\\|\"';:/?.>,<`~ " };
        for (items) |it| {
            std.debug.print("test password '{s}'\n", .{it});
            try password(it);
        }
    }

    {
        const items = [_][]const u8{ "a☰23234sd2fsf", " 1ac" };
        for (items) |it| {
            password(it) catch {
                std.debug.print("test password '{s}'\n", .{it});
                try std.testing.expect(true);
            };
        }
    }
}
