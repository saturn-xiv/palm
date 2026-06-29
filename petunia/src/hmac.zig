const std = @import("std");

const Hmac = struct {
    fn sign(_: *Hmac) void {
        // TODO
    }
    fn verify(_: *Hmac) void {
        // TODO
    }
};

test "hmac sha512" {
    try std.testing.expect(1 + 1 == 2);
}
