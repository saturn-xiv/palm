const std = @import("std");

const PostgreSql = struct {
    fn sign(_: *PostgreSql) void {
        // TODO
    }
    fn verify(_: *PostgreSql) void {
        // TODO
    }
};

test "postgresql" {
    try std.testing.expect(1 + 1 == 2);
}
