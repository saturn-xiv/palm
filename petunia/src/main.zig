const std = @import("std");
const petunia = @import("petunia");
const config = @import("config");
const third = @cImport({
    @cInclude("time.h");
});

pub fn main() !void {
    log_level = .debug;
    try petunia.init();

    // Prints to stderr, ignoring potential errors.
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
    std.debug.print("version: {s}, sqlite3: {s}\n", .{ config.version, petunia.third.SQLITE_VERSION });
    try petunia.bufferedPrint();
}

pub const std_options: std.Options = .{
    .logFn = init_logger,
};

var log_level: std.log.Level = .debug;

fn init_logger(
    comptime level: std.log.Level,
    comptime scope: @TypeOf(.enum_literal),
    comptime format: []const u8,
    args: anytype,
) void {
    if (@intFromEnum(level) <= @intFromEnum(log_level)) {
        std.log.defaultLog(level, scope, format, args);
    }

    // const level_txt = comptime level.asText();
    // const prefix = if (scope == .default) ": " else "(" ++ @tagName(scope) ++ "): ";

    // // const ts = std.time.milliTimestamp();
    // const now: third.time_t = third.time(null);
    // // const now_str = third.ctime(&now);
    // const time_info = third.localtime(&now);
    // var buf: [64]u8 = undefined;
    // const len = third.strftime(&buf, buf.len, "%Y-%m-%d %H:%M:%S", time_info);

    // nosuspend std.debug.print("[{s}] {s}{s}", .{ petunia.now(), level_txt, prefix });
    // nosuspend std.debug.print(format ++ "\n", args);
}

test "simple test" {
    std.debug.print("version: {s}\n", .{"123"});

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
