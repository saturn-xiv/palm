namespace cpp tuberose.v1
namespace java com.github.saturn_xiv.palm.plugins.tuberose.v1

struct SmsSendTask {
    1:required set<string> to;
    2:required string body;
    9:optional string callback;
}
