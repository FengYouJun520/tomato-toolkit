/// 获取类名
///
/// - class_name: 全类名
pub fn get_simple_name(class_name: &str) -> Option<String> {
    if class_name.is_empty() {
        return None;
    }

    class_name
        .rfind(|name| name == '.')
        .map(|index| class_name[index + 1..].to_string())
}
