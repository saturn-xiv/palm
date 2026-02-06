const std = @import("std");

pub const Currency = struct {
    name: []const u8,
    country: []const u8,
    code: []const u8,
    number: u16,
    units: *u8,
};

pub fn all(allocator: std.mem.Allocator) ![]Currency {
    _ = @embedFile("iso4217/list-one.xml");

    var iso4217: std.ArrayList(Currency) = .empty;

    return iso4217.toOwnedSlice(allocator);
}

test "parse iso4217 list-one.xml" {
    const allocator = std.testing.allocator;

    const items = try all(allocator);
    defer allocator.free(items);
    for (items) |it| {
        std.debug.print("Found currency: {s} {s} {s}\n", .{ it.code, it.country, it.name });
    }
}
