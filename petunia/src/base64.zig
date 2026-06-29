const std = @import("std");

pub fn encode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
    const len = std.base64.standard.Encoder.calcSize(data.len);
    const buf = try allocator.alloc(u8, len);
    return std.base64.standard.Encoder.encode(buf, data);
}
