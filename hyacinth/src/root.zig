const std = @import("std");
const begonia = @import("begonia");
const getopt = @cImport({
    @cInclude("getopt.h");
});

pub fn launch(version: []const u8, args: [][:0]u8) !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var config_file: std.ArrayList(u8) = .empty;
    defer config_file.deinit(allocator);
    {
        try config_file.appendSlice(allocator, "config.toml");
        try config_file.append(allocator, 0);
    }

    var argv: std.ArrayList([*c]u8) = .empty;
    defer argv.deinit(allocator);
    for (args) |it| {
        try argv.append(allocator, it);
    }

    const help =
        \\Usage: hyacinth [GLOBAL-OPTIONS] <COMMAND> [OPTIONS]
        \\
        \\Global options:
        \\  -h --help           Show usage.
        \\  -v --version        Print version.
        \\  -c --config CONFIG  Load configuration file(config.toml).
        \\
        \\Commands:
        \\  http                  Start a HTTP server.
        \\  create-user-by-email  Create an email user.
        \\  apply-role-to-user    Apply a role(by code) to a user(by sn).
        \\  revoke-role-from-user Revoke a role(by code) from a user(by sn).
        \\
        \\You can also visit `https://github.com/saturn-xiv/palm` to get more information.
    ;

    const options = [_]getopt.option{
        .{ .name = "help", .has_arg = getopt.no_argument, .flag = 0, .val = 'h' },
        .{ .name = "version", .has_arg = getopt.no_argument, .flag = 0, .val = 'v' },
        .{ .name = "config", .has_arg = getopt.required_argument, .flag = 0, .val = 'c' },
    };

    const argc = @as(c_int, @intCast(argv.items.len));
    while (true) {
        const opt = getopt.getopt_long(argc, argv.items.ptr, "hvc:p:u:r:", options[0..], null);
        switch (opt) {
            'h' => {
                std.debug.print("{s}\n", .{help});
                return;
            },
            'v' => {
                std.debug.print("{s}\n", .{version});
                return;
            },
            'c' => {
                config_file.clearAndFree(allocator);
                try config_file.appendSlice(allocator, std.mem.span(getopt.optarg));
                try config_file.append(allocator, 0);
            },
            '?' => {
                return begonia.Error.InvalidArguments;
            },
            else => {
                break;
            },
        }
    }

    try begonia.init();
    defer _ = begonia.destroy() catch |err| {
        std.debug.print("an error occurred while close logger: {}\n", .{err});
    };

    const logger = begonia.third.log4c_category_get("main");
    begonia.third.info(logger, "load configuration from %s", config_file.items.ptr);
}
