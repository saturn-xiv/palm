const std = @import("std");
pub const third = @cImport({
    @cInclude("third.h");
});
pub const Error = error{ ConnectFailed, NotSingleRowResult, NotSingleColumnResult };

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
    {
        const len = third.PQntuples(res);
        if (len != 1) {
            return Error.NotSingleRowResult;
        }
    }
    {
        const len = third.PQnfields(res);
        if (len != 1) {
            return Error.NotSingleRowResult;
        }
    }
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

    // const len = @as(usize, @intCast(third.PQgetlength(result, tup_num, field_num)));
    // const buf = try allocator.alloc(u8, len);
    // const tmp = third.PQgetvalue(result, tup_num, field_num);
    // if (tmp[0] == '\0') {
    //     return null;
    // }
    // @memcpy(buf, tmp[0..len]);
    // return buf;
}

// pub fn is_null(result: *third.PGresult, tup_num: c_int, field_num: c_int) bool {
//     const tmp = third.PQgetvalue(result, tup_num, field_num);
//     return tmp[0] == '\0';
// }

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
}
