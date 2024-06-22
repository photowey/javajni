/*
 * Copyright © 2024 the original author or authors_
 *
 * Licensed under the Apache License, Version 2_0 (the "License");
 * you may not use this file except in compliance with the License_
 * You may obtain a copy of the License at
 *
 *     http:///www_apache_org/licenses/LICENSE-2_0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied_
 * See the License for the specific language governing permissions and
 * limitations under the License_
 */

/// ----------------------------------------------------------------

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint};

/// ----------------------------------------------------------------
#[cfg(test)]
mod tests;

/// ----------------------------------------------------------------

/// Add two integers.
///
/// Java method: io.github.photowey.rust.jni.in.action.lib.Javajni#add0
/// # Java Class:
/// ```java
/// public class Javajni {
///
///     static {
///         // SET: java.library.path
///         System.loadLibrary("javajni");
///     }
///
///     public static int add(int left, int right) {
///         return add0(left, right);
///     }
///
///     public static native int add0(int left, int right);
/// }
/// ```
#[no_mangle]
pub extern "system" fn Java_io_github_photowey_rust_jni_in_action_lib_Javajni_add0(
    _env: JNIEnv,
    _class: JClass,
    left: jint,
    right: jint) -> jint {
    add(left, right)
}

// ----------------------------------------------------------------

/// Say hello~
///
/// |- Input: String
///
/// |- Output: String
///
/// Java method: io.github.photowey.rust.jni.in.action.lib.Javajni#hello0
#[no_mangle]
pub extern "system" fn Java_io_github_photowey_rust_jni_in_action_lib_Javajni_hello0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    output
}

/// Say hello~
///
/// |- Input: String
///
/// |- Output: String
///
/// Java method: io.github.photowey.rust.jni.in.action.lib.Javajni#uppercase0
#[no_mangle]
pub extern "system" fn Java_io_github_photowey_rust_jni_in_action_lib_Javajni_uppercase0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    let output = input.to_uppercase();

    let output = env
        .new_string(output)
        .expect("Couldn't create java string!");
    output
}

// ----------------------------------------------------------------

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

