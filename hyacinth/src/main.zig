const std = @import("std");
const config = @import("config");
const hyacinth = @import("hyacinth");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    try hyacinth.launch(allocator, config.version, args);
}
