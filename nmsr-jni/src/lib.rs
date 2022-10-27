mod error_handling;

use crate::error_handling::{get_string_or_throw, unwrap_or_throw_java_exception};
use image::ImageOutputFormat::Png;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyteArray, jlong, JNI_TRUE};
use jni::JNIEnv;
use nmsr_lib::parts::manager::PartsManager;
use nmsr_lib::rendering::entry::RenderingEntry;
use std::io::{BufWriter, Cursor};

#[no_mangle]
pub extern "system" fn Java_io_github_nickacpt_jnmsr_natives_NMSRNatives_initialize(
    env: JNIEnv,
    _class: JClass,
    parts_path: JString,
) -> jlong {
    let parts_path = get_string_or_throw!(env, parts_path, 0);

    let parts_manager = unwrap_or_throw_java_exception!(env, PartsManager::new(&parts_path), 0);

    Box::into_raw(Box::from(parts_manager)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_io_github_nickacpt_jnmsr_natives_NMSRNatives_renderSkin(
    env: JNIEnv,
    _class: JClass,
    parts_manager_ptr: jlong,
    skin_bytes: jbyteArray,
    slim_arms: jboolean,
) -> jbyteArray {
    // Create an empty byte array to return if something goes wrong
    let empty_byte_array = env.new_byte_array(0).expect("NewByteArray should not fail");

    // Get the parts manager from the raw pointer
    let parts_manager = unsafe { &*(parts_manager_ptr as *const PartsManager) };
    let slim_arms = slim_arms == JNI_TRUE;

    // Get the skin bytes from the Java array
    let skin_bytes =
        unwrap_or_throw_java_exception!(env, env.convert_byte_array(skin_bytes), empty_byte_array);
    // Load the skin as an image from the bytes
    let skin_image = unwrap_or_throw_java_exception!(
        env,
        image::load_from_memory(skin_bytes.as_slice()),
        empty_byte_array
    )
    .into_rgba16();

    // Create a new rendering entry
    let entry = RenderingEntry::new(skin_image, slim_arms);

    // Render the skin
    let render =
        unwrap_or_throw_java_exception!(env, entry.render(parts_manager), empty_byte_array);
    let mut render_bytes = Vec::new();

    // Write the image to a byte array
    {
        let mut writer = BufWriter::new(Cursor::new(&mut render_bytes));
        unwrap_or_throw_java_exception!(env, render.write_to(&mut writer, Png), empty_byte_array);
    }

    // Create a new Java byte array from the output bytes
    unwrap_or_throw_java_exception!(
        env,
        env.byte_array_from_slice(render_bytes.as_slice()),
        empty_byte_array
    )
}

#[no_mangle]
pub extern "system" fn Java_io_github_nickacpt_jnmsr_natives_NMSRNatives_destroy(
    _env: JNIEnv,
    _class: JClass,
    parts_manager_ptr: jlong,
) {
    let _ = unsafe { Box::from_raw(parts_manager_ptr as *mut PartsManager) };
}