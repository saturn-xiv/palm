const std = @import("std");
pub const validator = @import("validator.zig");
pub const cache = @import("cache.zig");
pub const orm = @import("orm.zig");
pub const queue = @import("queue.zig");
pub const http = @import("http.zig");
pub const crypto = @import("crypto.zig");
pub const portal = @import("plugins-portal.zig");
pub const cms = @import("plugins-cms.zig");
pub const blog = @import("plugins-blog.zig");
pub const forum = @import("plugins-forum.zig");
pub const babel = @import("plugins-babel.zig");
pub const accounting = @import("plugins-accounting.zig");

pub const third = @cImport({
    @cInclude("third.h");
});

pub const Error = error{ InvalidArguments, BadRequest };

pub fn start_http_server(port: u16) !void {
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
