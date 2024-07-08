use std::ptr::null_mut;

use crate::il2cpp::{symbols::{get_field_from_name, set_field_object_value}, types::*};

static mut TEXT_FIELD: *mut FieldInfo = null_mut();
pub fn set_Text(this: *mut Il2CppObject, value: *mut Il2CppString) {
    set_field_object_value(this, unsafe { TEXT_FIELD }, value);
}

pub fn init(StoryTimelineTextClipData: *mut Il2CppClass) {
    find_nested_class_or_return!(StoryTimelineTextClipData, ChoiceData);

    unsafe {
        TEXT_FIELD = get_field_from_name(ChoiceData, c"Text");
    }
}