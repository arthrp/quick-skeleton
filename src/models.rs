use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct TemplateParameter{
    pub name: String,
    pub value: String,
    pub desc: String
}
