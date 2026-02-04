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
    mod.addCSourceFile(.{ .file = b.path("src/third.c") });

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

    mod.linkSystemLibrary("log4c", .{});
    mod.linkSystemLibrary("mysqlclient", .{});
    mod.linkSystemLibrary("pq", .{});
    mod.linkSystemLibrary("hiredis", .{});
    mod.linkSystemLibrary("rabbitmq", .{});
}
