const std = @import("std");
pub const third = @cImport({
    @cInclude("third.h");
});
