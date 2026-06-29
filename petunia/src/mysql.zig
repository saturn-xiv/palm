const std = @import("std");

const MySql = struct {
    fn sign(_: *MySql) void {
        // TODO
    }
    fn verify(_: *MySql) void {
        // TODO
    }
};

test "mysql" {
    try std.testing.expect(1 + 1 == 2);
}
