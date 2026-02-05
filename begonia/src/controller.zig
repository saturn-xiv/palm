const std = @import("std");
const third = @cImport({
    @cInclude("third.h");
});

pub fn open() !void {
    //
    // third.log(logger, third.LOG4C_PRIORITY_ERROR, "error");
    // return logger;
}
