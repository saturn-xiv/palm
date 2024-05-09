namespace cpp lily.v1
namespace java com.github.saturn_xiv.palm.plugins.lily.v1

struct TexToPdfTask {
    1:required string bucket,
    2:required string object,
    3:required Tex tex,
    9:optional string callback,
}

struct Tex {
    1:required binary homepage,
    2:required map<string,binary> files,
}
