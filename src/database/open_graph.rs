#[crud_table(table_name:"ewd.lnk_opengraph" | table_columns:"log_id,log_link_id,log_tag,log_content")]
#[derive(Clone, Debug)]
pub struct OpenGraph {
    pub log_id: i32,
    pub log_link_id: i32,
    pub log_tag: String,
    pub log_content: String,
}