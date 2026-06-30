const std = @import("std");

pub const StandardNoPadding = struct {
    pub fn encode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
        const len = std.base64.standard_no_pad.Encoder.calcSize(data.len);
        const buf = try allocator.alloc(u8, len);
        _ = std.base64.standard_no_pad.Encoder.encode(buf, data);
        return buf;
    }
    pub fn decode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
        const len = try std.base64.standard_no_pad.Decoder.calcSizeForSlice(data);
        const buf = try allocator.alloc(u8, len);

        try std.base64.standard_no_pad.Decoder.decode(buf, data);
        return buf;
    }
};

pub const StandardWithPadding = struct {
    pub fn encode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
        const len = std.base64.standard.Encoder.calcSize(data.len);
        const buf = try allocator.alloc(u8, len);
        _ = std.base64.standard.Encoder.encode(buf, data);
        return buf;
    }
    pub fn decode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
        const len = try std.base64.standard.Decoder.calcSizeForSlice(data);
        const buf = try allocator.alloc(u8, len);

        try std.base64.standard.Decoder.decode(buf, data);
        return buf;
    }
};

pub const UrlSafeNoPadding = struct {
    pub fn encode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
        const len = std.base64.url_safe_no_pad.Encoder.calcSize(data.len);
        const buf = try allocator.alloc(u8, len);
        _ = std.base64.url_safe_no_pad.Encoder.encode(buf, data);
        return buf;
    }
    pub fn decode(allocator: std.mem.Allocator, data: []const u8) ![]const u8 {
        const len = try std.base64.url_safe_no_pad.Decoder.calcSizeForSlice(data);
        const buf = try allocator.alloc(u8, len);

        try std.base64.url_safe_no_pad.Decoder.decode(buf, data);
        return buf;
    }
};
