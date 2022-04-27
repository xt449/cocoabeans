use crate::cocoabeans::protocol::versions::*;

// TODO - make rust happy
/*const MANAGER: std::collections::HashMap<i32, &'static dyn ProtocolVersion> = get_default_map();

const fn get_default_map() -> std::collections::HashMap<i32, &'static dyn ProtocolVersion> {
    let mut map: std::collections::HashMap<i32, &'static dyn ProtocolVersion> =
        std::collections::HashMap::new();
    map[&758] = &V758 {};
    return map;
}

pub fn register_protocol_version(version: i32, protocol: &'static dyn ProtocolVersion) {
    MANAGER[&version] = protocol;
}*/

const DEFAULT_VERSION: V758 = V758 {};

pub fn get_protocol_version(version: i32) -> Option<&'static dyn ProtocolVersion> {
    return Some(&DEFAULT_VERSION);
    /*return if MANAGER.contains_key(&version) {
        Some(MANAGER[&version])
    } else {
        None
    };*/
}
