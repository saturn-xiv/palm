const std = @import("std");
const config = @import("config");

const petunia = @import("petunia");

pub fn main(init: std.process.Init) !void {
    const allocator: std.mem.Allocator = init.arena.allocator();

    try petunia.sodium.init();
    try petunia.logging.init();
    defer petunia.logging.release();

    const logger = try petunia.logging.get_category(allocator, "petunia");
    try petunia.logging.debug(allocator, logger, "run on debug mode {s}", .{config.version});

    const args = try init.minimal.args.toSlice(allocator);
    for (args) |arg| {
        std.log.info("arg: {s}", .{arg});
    }
}
