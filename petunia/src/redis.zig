const std = @import("std");

const Cluster = struct {
    fn sign(_: *Cluster) void {
        // TODO
    }
    fn verify(_: *Cluster) void {
        // TODO
    }
};

test "redis cluster" {
    try std.testing.expect(1 + 1 == 2);
}

const Standalone = struct {
    fn sign(_: *Standalone) void {
        // TODO
    }
    fn verify(_: *Standalone) void {
        // TODO
    }
};

test "redis standalone" {
    try std.testing.expect(1 + 1 == 2);
}
