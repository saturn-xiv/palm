const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const mod = b.addModule("begonia", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    mod.addIncludePath(b.path("src"));
    mod.addCSourceFiles(.{ .files = &.{
        "src/validator.c",
        "src/logger.c",
        "src/iso4217.c",
        "src/http.c",
        "src/postgresql.c",
        "src/mysql.c",
        "src/redis.c",
        "src/rabbitmq.c",
    } });

    mod.addSystemIncludePath(.{ .cwd_relative = "/usr/include" });

    {
        const cpu_arch = target.result.cpu.arch;
        if (cpu_arch == .x86_64) {
            mod.addSystemIncludePath(.{ .cwd_relative = "/usr/include/x86_64-linux-gnu" });
            mod.addLibraryPath(.{ .cwd_relative = "/usr/lib/x86_64-linux-gnu" });
        } else if (cpu_arch.isAARCH64()) {
            mod.addSystemIncludePath(.{ .cwd_relative = "/usr/include/aarch64-linux-gnu" });
            mod.addLibraryPath(.{ .cwd_relative = "/usr/lib/aarch64-linux-gnu" });
        }
    }

    mod.linkSystemLibrary("expat", .{ .preferred_link_mode = .static });
    mod.linkSystemLibrary("log4c", .{ .preferred_link_mode = .static });
    mod.linkSystemLibrary("hiredis", .{ .preferred_link_mode = .static });
    mod.linkSystemLibrary("rabbitmq", .{ .preferred_link_mode = .static });
    mod.linkSystemLibrary("sodium", .{ .preferred_link_mode = .static });
    mod.linkSystemLibrary("mysqlclient", .{});
    mod.linkSystemLibrary("pq", .{});

    const mod_tests = b.addTest(.{
        .root_module = mod,
    });
    mod_tests.linkLibC();
    const run_mod_tests = b.addRunArtifact(mod_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_mod_tests.step);
}
