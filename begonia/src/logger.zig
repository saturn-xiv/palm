const third = @cImport({
    @cInclude("third.h");
});

pub fn aaa() !void {
    _ = third.log4c_init();
    const logger = third.log4c_category_get("root");

    third.log(logger, third.LOG4C_PRIORITY_DEBUG, "log4c(v%s)", third.log4c_version());

    third.log(logger, third.LOG4C_PRIORITY_DEBUG, "debug");
    third.log(logger, third.LOG4C_PRIORITY_INFO, "info");
    third.log(logger, third.LOG4C_PRIORITY_ERROR, "error");

    _ = third.log4c_fini();
}
