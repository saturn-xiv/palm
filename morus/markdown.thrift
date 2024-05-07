namespace cpp morus.v1.markdown
namespace java com.github.saturn_xiv.palm.plugins.morus.v1.markdown

service Markdown {
    string to_html(1:string text, 2:bool sanitize);
}
