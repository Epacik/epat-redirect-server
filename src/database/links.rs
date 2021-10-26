#[crud_table(table_name:"ewd.lnk_links" | table_columns:"lnk_id,lnk_path,lnk_target,lnk_hide_target")]
#[derive(Clone, Debug)]
pub struct Links{
    pub lnk_id: i32,
    pub lnk_path: String,
    pub lnk_target: String,
    pub lnk_hide_target: i32,
}
