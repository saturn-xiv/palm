const third = @cImport({
    @cInclude("third.h");
});

pub fn aaa() !void {

    //

    // third.log(logger, third.LOG4C_PRIORITY_DEBUG, "debug");
    // third.log(logger, third.LOG4C_PRIORITY_INFO, "info");
    // third.log(logger, third.LOG4C_PRIORITY_ERROR, "error");

}

pub fn open() !void {
    //
    // third.log(logger, third.LOG4C_PRIORITY_ERROR, "error");
    // return logger;
}
