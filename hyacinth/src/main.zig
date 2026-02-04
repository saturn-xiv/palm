const std = @import("std");
const config = @import("config");
const hyacinth = @import("hyacinth");

const third = @cImport({
    @cInclude("third.h");
});

pub fn main() !void {
    _ = third.log4c_init();
    const logger = third.log4c_category_get("root");

    third.log(logger, third.LOG4C_PRIORITY_DEBUG, "log4c(v%s)", third.log4c_version());

    third.log(logger, third.LOG4C_PRIORITY_DEBUG, "debug");
    third.log(logger, third.LOG4C_PRIORITY_INFO, "info");
    third.log(logger, third.LOG4C_PRIORITY_ERROR, "error");

    std.debug.print("version: {s}\n", .{config.version});
    _ = third.log4c_fini();
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}
