namespace cpp lily.v1
namespace java com.github.saturn_xiv.palm.plugins.lily.v1

struct TexToPdfTask {
    1:string bucket,
    2:string object,
    3:Tex tex,
    9:optional string callback,
}

struct Tex {
    1:binary homepage,
    2:map<string,binary> files,
}
