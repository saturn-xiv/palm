const std = @import("std");
pub const validator = @import("validator.zig");

pub const third = @cImport({
    @cInclude("third.h");
});

pub const Error = error{ InvalidArguments, BadRequest };

pub fn http(port: u16) !void {
    const logger = third.log4c_category_get("http");
    third.info(logger, "start a HTTP server listened on: http://0.0.0.0:%d", port);
}

pub fn init() !void {
    _ = third.log4c_init();
    const logger = third.log4c_category_get("root");
    third.debug(logger, "log4c(v%s)", third.log4c_version());
}

pub fn destroy() !void {
    _ = third.log4c_fini();
}

test {
    @import("std").testing.refAllDecls(@This());
}
