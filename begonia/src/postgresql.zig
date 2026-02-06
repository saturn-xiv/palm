const std = @import("std");
pub const third = @cImport({
    @cInclude("third.h");
});

pub const Timestamp = struct {
    time: third.tm = .{},
    microseconds: i64 = 0,
};

pub const Error = error{ ConnectFailed, BadTimestampField };

// https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
pub fn open(allocator: std.mem.Allocator, host: []const u8, port: u16, user: []const u8, password: []const u8, db_name: []const u8) !?*third.PGconn {
    const logger = third.log4c_category_get("postgresql");
    const url = try std.fmt.allocPrint(allocator, "host={s} port={d} user={s} password={s} dbname={s} sslmode=disable connect_timeout=10", .{ host, port, user, password, db_name });
    defer allocator.free(url);
    third.debug(logger, "open postgresql://%$s@%$s:%d/%s", user.ptr, host.ptr, port, db_name.ptr);
    const db = third.PQconnectdb(url.ptr);
    errdefer third.PQfinish(db);

    if (third.PQstatus(db) != third.CONNECTION_OK) {
        third.error_(logger, "%s", third.PQerrorMessage(db));
        return Error.ConnectFailed;
    }

    return db;
}

pub fn version(allocator: std.mem.Allocator, db: ?*third.PGconn) !?[]const u8 {
    const logger = third.log4c_category_get("postgresql");
    const res = third.PQexec(db.?, "SELECT VERSION()");
    defer third.PQclear(res);

    if (third.PQresultStatus(res) != third.PGRES_TUPLES_OK) {
        third.error_(logger, "%s", third.PQerrorMessage(db));
    }
    // {
    //     const len = third.PQntuples(res);
    //     if (len != 1) {
    //         return Error.NotSingleRowResult;
    //     }
    // }
    // {
    //     const len = third.PQnfields(res);
    //     if (len != 1) {
    //         return Error.NotSingleRowResult;
    //     }
    // }
    return string(allocator, res, 0, 0);
}

pub fn string(allocator: std.mem.Allocator, result: ?*third.PGresult, tup_num: c_int, field_num: c_int) !?[]const u8 {
    const tmp = std.mem.span(third.PQgetvalue(result, tup_num, field_num));
    if (tmp.len == 0) {
        return null;
    }
    const buf = try allocator.alloc(u8, tmp.len);
    @memcpy(buf, tmp);
    return buf;
}

pub fn int64(result: ?*third.PGresult, tup_num: c_int, field_num: c_int) !?i64 {
    const tmp = std.mem.span(third.PQgetvalue(result, tup_num, field_num));
    if (tmp.len == 0) {
        return null;
    }
    const it: i64 = try std.fmt.parseInt(i64, tmp, 10);
    return it;
}

pub fn timestamp(result: ?*third.PGresult, tup_num: c_int, field_num: c_int) !?Timestamp {
    const tmp = std.mem.span(third.PQgetvalue(result, tup_num, field_num));
    if (tmp.len == 0) {
        return null;
    }
    var it: Timestamp = .{};
    if (third.postgresql_timestamp(tmp.ptr, &it.time, &it.microseconds) != 0) {
        return Error.BadTimestampField;
    }
    return it;
}

// CREATE USER www WITH PASSWORD 'change-me';
// CREATE DATABASE begonia_testing WITH OWNER www;
test "postgresql" {
    const allocator = std.testing.allocator;
    const db = try open(allocator, "127.0.0.1", 5432, "www", "change-me", "begonia_testing");
    defer third.PQfinish(db);
    {
        const it = try version(allocator, db);
        defer allocator.free(it.?);
        std.debug.print("PostgreSql: {s}\n", .{it.?});
    }
    {
        const res = third.PQexec(db.?, "SELECT 1");
        defer third.PQclear(res);
        try std.testing.expectEqual(third.PQresultStatus(res), third.PGRES_TUPLES_OK);
        const it = try int64(res, 0, 0);
        try std.testing.expectEqual(it.?, 1);
    }
    {
        std.debug.print("Timestamp(second): {d}\n", .{std.time.timestamp()});
        std.debug.print("Timestamp(microsecond): {d}\n", .{std.time.microTimestamp()});
    }
    {
        // 2026-02-06 03:35:18.599259+00
        const res = third.PQexec(db.?, "SELECT CURRENT_TIMESTAMP");
        defer third.PQclear(res);
        const it = try timestamp(res, 0, 0);
        std.debug.print("Current Timestamp: ({d}, {d}, {d}) {d}\n", .{
            it.?.time.tm_year + 1900,
            it.?.time.tm_mon + 1,
            it.?.time.tm_mday,
            it.?.microseconds,
        });
    }
}
