const std = @import("std");

const Jwt = struct {
    fn sign(_: *Jwt) void {
        // TODO
    }
    fn verify(_: *Jwt) void {
        // TODO
    }
};

test "jwt sha512" {
    try std.testing.expect(1 + 1 == 2);
}
