use jni::objects::JObject;
use jni::JNIEnv;
use jni::errors::Result as JNIResult;
use systest::get_jvm;
use systest::print_exception;
use systest::wrappers::*;

fn string_to_jobject<'a>(env: &JNIEnv<'a>, string: &str) -> JNIResult<JObject<'a>> {
    Ok(JObject::from(env.new_string(string.to_owned())?))
}

#[test]
#[should_panic(expected = "Java binding type failure. Expected object of class java/lang/Error, but got java/lang/Integer instead")]
fn method_should_fail_on_wrong_receiver() {
    let jvm = get_jvm().expect("failed go get jvm reference");

    let env = jvm
        .attach_current_thread()
        .expect("failed to attach jvm thread");

    env.with_local_frame(16, || {
        let error_wrapper = java::lang::Error::with(&env);
        let integer_object = java::lang::Integer::with(&env).new(1337)?;
        error_wrapper.call_getMessage(integer_object)?;
        Ok(JObject::null())
    }).unwrap_or_else(|e| {
        print_exception(&env);
        panic!("{} source: {:?}", e, std::error::Error::source(&e));
    });
}

#[test]
#[should_panic(expected = "Java binding type failure. Expected object of class java/lang/Integer, but got java/lang/Error instead")]
fn method_should_fail_on_wrong_argument() {
    let jvm = get_jvm().expect("failed go get jvm reference");

    let env = jvm
        .attach_current_thread()
        .expect("failed to attach jvm thread");

    env.with_local_frame(16, || {
        let integer_wrapper = java::lang::Integer::with(&env);
        let integer_object = integer_wrapper.new(1337)?;
        let error_wrapper = java::lang::Error::with(&env);
        let error_object = error_wrapper.new(string_to_jobject(&env, "error message")?)?;
        let _result = integer_wrapper.call_compareTo(integer_object, error_object);
        Ok(JObject::null())
    }).unwrap_or_else(|e| {
        print_exception(&env);
        panic!("{} source: {:?}", e, std::error::Error::source(&e));
    });
}
