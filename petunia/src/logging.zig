const std = @import("std");

const c = @cImport({
    @cDefine("_NO_CRT_STDIO_INLINE", "1");
    @cInclude("stdio.h");
    @cInclude("stdlib.h");
    @cInclude("zlog.h");
});

pub const Error = error{
    ZlogInit,
    ZlogGetCategory,
};

pub fn debug(allocator: std.mem.Allocator, category: *c.zlog_category_t, comptime fmt: []const u8, args: anytype) !void {
    try log(allocator, category, c.ZLOG_LEVEL_DEBUG, fmt, args);
}
pub fn info(allocator: std.mem.Allocator, category: *c.zlog_category_t, comptime fmt: []const u8, args: anytype) !void {
    try log(allocator, category, c.ZLOG_LEVEL_INFO, fmt, args);
}
pub fn warning(allocator: std.mem.Allocator, category: *c.zlog_category_t, comptime fmt: []const u8, args: anytype) !void {
    try log(allocator, category, c.ZLOG_LEVEL_WARN, fmt, args);
}
pub fn error_(allocator: std.mem.Allocator, category: *c.zlog_category_t, comptime fmt: []const u8, args: anytype) !void {
    try log(allocator, category, c.ZLOG_LEVEL_ERROR, fmt, args);
}

fn log(allocator: std.mem.Allocator, category: *c.zlog_category_t, level: c_int, comptime fmt: []const u8, args: anytype) !void {
    const buf = try std.fmt.allocPrint(allocator, fmt, args);
    defer allocator.free(buf);

    c.zlog(category, "", 0, "", 0, 0, level, buf.ptr);
}

pub fn get_category(allocator: std.mem.Allocator, name: []const u8) !*c.zlog_category_t {
    const it = try allocator.dupeZ(u8, name);
    defer allocator.free(it);

    const rc = c.zlog_get_category(it.ptr) orelse return Error.ZlogGetCategory;
    return rc;
}

pub fn init() !void {
    const rc = c.zlog_init("zlog.conf");
    if (rc != c.EXIT_SUCCESS) {
        return Error.ZlogInit;
    }
}

pub fn release() void {
    c.zlog_fini();
}
