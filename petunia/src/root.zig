const std = @import("std");
pub const cache = @import("cache.zig");
pub const third = @cImport({
    @cInclude("time.h");
    @cInclude("sqlite3.h");
    @cInclude("hiredis/hiredis.h");
    // @cInclude("fio.h");
    // @cInclude("http.h");
});

pub fn init() !void {
    // if (debug) {
    //     log_level = .debug;
    // } else {
    //     log_level = .info;
    // }
    std.log.debug("sqlite3: v{s}", .{third.SQLITE_VERSION});
}

pub fn now() []u8 {
    const it: third.time_t = third.time(null);
    const ti = third.localtime(&it);
    var buf: [64]u8 = undefined;
    const len = third.strftime(&buf, buf.len, "%Y-%m-%d %H:%M:%S", ti);
    return buf[0..len];
}
// ----------------------------------------------------------------------------

pub fn bufferedPrint() !void {
    // Stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try stdout.flush(); // Don't forget to flush!
}

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "basic add functionality" {
    try std.testing.expect(add(3, 7) == 10);
}
