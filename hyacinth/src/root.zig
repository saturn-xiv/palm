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

    var user_sn: std.ArrayList(u8) = .empty;
    defer user_sn.deinit(allocator);

    var role_code: std.ArrayList(u8) = .empty;
    defer role_code.deinit(allocator);

    var theme_name: std.ArrayList(u8) = .empty;
    defer theme_name.deinit(allocator);
    {
        try theme_name.appendSlice(allocator, "bootstrap5");
        try theme_name.append(allocator, 0);
    }

    var email: std.ArrayList(u8) = .empty;
    defer email.deinit(allocator);

    var username: std.ArrayList(u8) = .empty;
    defer username.deinit(allocator);

    var password: std.ArrayList(u8) = .empty;
    defer password.deinit(allocator);

    var argv: std.ArrayList([*c]u8) = .empty;
    defer argv.deinit(allocator);
    for (args) |it| {
        try argv.append(allocator, it);
    }

    var port: u16 = 8080;

    const help =
        \\Usage: hyacinth [OPTIONS] <COMMAND>
        \\
        \\General options:
        \\  -h --help          Show usage.
        \\  -v --version       Print version.
        \\  -c --config CONFIG Load configuration file(config.toml).
        \\
        \\Commands:
        \\  http                  Start a HTTP server.
        \\  create-user-by-email  Create an email user.
        \\  apply-role-to-user    Apply a role(by code) to a user(by sn).
        \\  revoke-role-from-user Revoke a role(by code) from a user(by sn).
        \\  list-user             List all users.
        \\
        \\Command(http):
        \\  -p --port  8080
        \\  -t --theme THEME(bootstrap5)
        \\
        \\Command(create-user-by-email):
        \\  -n --name  NAME
        \\  -e --email EMAIL
        \\  -P --password PASSWORD
        \\
        \\Command(apply-role-to-user, revoke-role-from-user):
        \\  -u --user USER_SN
        \\  -r --role ROLE_CODE
        \\
        \\You can also visit `https://github.com/saturn-xiv/palm` to get more information.
    ;

    const options = [_]getopt.option{
        .{ .name = "help", .has_arg = getopt.no_argument, .flag = 0, .val = 'h' },
        .{ .name = "version", .has_arg = getopt.no_argument, .flag = 0, .val = 'v' },
        .{ .name = "config", .has_arg = getopt.required_argument, .flag = 0, .val = 'c' },
        .{ .name = "port", .has_arg = getopt.required_argument, .flag = 0, .val = 'p' },
        .{ .name = "theme", .has_arg = getopt.required_argument, .flag = 0, .val = 't' },
        .{ .name = "name", .has_arg = getopt.required_argument, .flag = 0, .val = 'n' },
        .{ .name = "email", .has_arg = getopt.required_argument, .flag = 0, .val = 'e' },
        .{ .name = "password", .has_arg = getopt.required_argument, .flag = 0, .val = 'P' },
        .{ .name = "user", .has_arg = getopt.required_argument, .flag = 0, .val = 'u' },
        .{ .name = "role", .has_arg = getopt.required_argument, .flag = 0, .val = 'r' },
    };

    const argc = @as(c_int, @intCast(argv.items.len));
    while (true) {
        const opt = getopt.getopt_long(argc, argv.items.ptr, "hvc:p:t::u:r:n:e:P:", options[0..], null);
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
            'p' => {
                port = try std.fmt.parseInt(u16, std.mem.span(getopt.optarg), 10);
            },
            't' => {
                theme_name.clearAndFree(allocator);
                try theme_name.appendSlice(allocator, std.mem.span(getopt.optarg));
                try theme_name.append(allocator, 0);
            },
            'n' => {
                username.clearAndFree(allocator);
                try username.appendSlice(allocator, std.mem.span(getopt.optarg));
                try username.append(allocator, 0);
            },
            'P' => {
                password.clearAndFree(allocator);
                try password.appendSlice(allocator, std.mem.span(getopt.optarg));
                try password.append(allocator, 0);
            },
            'e' => {
                email.clearAndFree(allocator);
                try email.appendSlice(allocator, std.mem.span(getopt.optarg));
                try email.append(allocator, 0);
            },
            'u' => {
                user_sn.clearAndFree(allocator);
                try user_sn.appendSlice(allocator, std.mem.span(getopt.optarg));
                try user_sn.append(allocator, 0);
            },
            'r' => {
                role_code.clearAndFree(allocator);
                try role_code.appendSlice(allocator, std.mem.span(getopt.optarg));
                try role_code.append(allocator, 0);
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

    if (getopt.optind < argc) {
        const optind = @as(usize, @intCast(getopt.optind));
        const cmd = args[optind];
        if (std.mem.eql(u8, cmd, "list-user")) {
            // TODO
            return;
        }
        if (std.mem.eql(u8, cmd, "http")) {
            // TODO
            begonia.third.info(logger, "start a HTTP server listened on http://0.0.0.0:%d", port);
            return;
        }
        if (std.mem.eql(u8, cmd, "apply-role-to-user")) {
            // TODO
            begonia.third.info(logger, "apply role(%s) to user(%s)", role_code.items.ptr, user_sn.items.ptr);
            return;
        }
        if (std.mem.eql(u8, cmd, "revoke-role-from-user")) {
            // TODO
            begonia.third.info(logger, "revoke role(%s) from user(%s)", role_code.items.ptr, user_sn.items.ptr);
            return;
        }
        if (std.mem.eql(u8, cmd, "create-user-by-email")) {
            const email_ = try begonia.validator.email(email.items);
            if (username.items.len == 0) {
                begonia.third.error_(logger, "empty username");
                return begonia.Error.BadRequest;
            }
            if (password.items.len == 0) {
                begonia.third.error_(logger, "not a valid password");
                return begonia.Error.BadRequest;
            }
            // TODO
            begonia.third.info(logger, "create an user(%s<%s>)", username.items.ptr, email_.ptr);
            return;
        }
        begonia.third.error_(logger, "unsupported sub-command: %s", cmd.ptr);
        return begonia.Error.InvalidArguments;
    }
}
