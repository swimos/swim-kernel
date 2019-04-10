extern crate swim_jvm;
extern crate swim_c_rt;

use swim_jvm::{JType, JArrayType, JMethodType};

#[test]
fn test_jtype_parse_primitive_type() {
    assert_eq!(JType::parse("V").unwrap(), JType::Void);
    assert_eq!(JType::parse("Z").unwrap(), JType::Boolean);
    assert_eq!(JType::parse("B").unwrap(), JType::Byte);
    assert_eq!(JType::parse("C").unwrap(), JType::Char);
    assert_eq!(JType::parse("S").unwrap(), JType::Short);
    assert_eq!(JType::parse("I").unwrap(), JType::Int);
    assert_eq!(JType::parse("J").unwrap(), JType::Long);
    assert_eq!(JType::parse("F").unwrap(), JType::Float);
    assert_eq!(JType::parse("D").unwrap(), JType::Double);
}

#[test]
fn test_jtype_primitive_type_as_jstr() {
    assert_eq!(JType::Void.to_bytes(), b"V\0" as &[u8]);
    assert_eq!(JType::Boolean.to_bytes(), b"Z\0" as &[u8]);
    assert_eq!(JType::Byte.to_bytes(), b"B\0" as &[u8]);
    assert_eq!(JType::Char.to_bytes(), b"C\0" as &[u8]);
    assert_eq!(JType::Short.to_bytes(), b"S\0" as &[u8]);
    assert_eq!(JType::Int.to_bytes(), b"I\0" as &[u8]);
    assert_eq!(JType::Long.to_bytes(), b"J\0" as &[u8]);
    assert_eq!(JType::Float.to_bytes(), b"F\0" as &[u8]);
    assert_eq!(JType::Double.to_bytes(), b"D\0" as &[u8]);
}

#[test]
fn test_jtype_parse_object_type() {
    assert_eq!(JType::parse("Ljava/lang/Object;").unwrap(),
               JType::object("Ljava/lang/Object;"));
}

#[test]
fn test_jtype_object_type_as_jstr() {
    assert_eq!(JType::object("Ljava/lang/Object;").to_bytes(), b"Ljava/lang/Object;\0" as &[u8]);
}

#[test]
fn test_jtype_parse_array_type() {
    assert_eq!(JType::parse("[B").unwrap(), JType::array("[B"));
    assert_eq!(JType::parse("[Ljava/lang/Object;").unwrap(),
               JType::array("[Ljava/lang/Object;"));
}

#[test]
fn test_jtype_array_type_as_jstr() {
    assert_eq!(JType::array("[B").to_bytes(), b"[B\0" as &[u8]);
    assert_eq!(JType::array("[Ljava/lang/Object;").to_bytes(), b"[Ljava/lang/Object;\0" as &[u8]);
}

#[test]
fn test_jtype_parse_method_type() {
    assert_eq!(JType::parse("()V").unwrap(), JType::method("()V", 2));
    assert_eq!(JType::parse("(I)Z").unwrap(), JType::method("(I)Z", 3));
    assert_eq!(JType::parse("(I[J)[D").unwrap(), JType::method("(I[J)[D", 5));
    assert_eq!(JType::parse("(I[Ljava/lang/Object;D)Ljava/lang/String;").unwrap(),
               JType::method("(I[Ljava/lang/Object;D)Ljava/lang/String;", 23));
}

#[test]
fn test_jtype_method_type_as_jstr() {
    assert_eq!(JType::method("()V", 2).to_bytes(), b"()V\0" as &[u8]);
    assert_eq!(JType::method("(I)Z", 3).to_bytes(), b"(I)Z\0" as &[u8]);
    assert_eq!(JType::method("(I[J)[D", 5).to_bytes(), b"(I[J)[D\0" as &[u8]);
    assert_eq!(JType::method("(I[Ljava/lang/Object;D)Ljava/lang/String;", 23).to_bytes(),
               b"(I[Ljava/lang/Object;D)Ljava/lang/String;\0" as &[u8]);
}

#[test]
fn test_jarraytype_to_component_type() {
    assert_eq!(JArrayType::parse("[B").unwrap().to_component_type(), JType::Byte);
    assert_eq!(JArrayType::parse("[[I").unwrap().to_component_type(), JType::array("[I"));
    assert_eq!(JArrayType::parse("[Ljava/lang/Object;").unwrap().to_component_type(),
               JType::object("Ljava/lang/Object;"));
}

#[test]
fn test_jmethodtype_to_return_type() {
    assert_eq!(JMethodType::parse("()V").unwrap().to_return_type(), JType::Void);
    assert_eq!(JMethodType::parse("(I)Z").unwrap().to_return_type(), JType::Boolean);
    assert_eq!(JMethodType::parse("(I[J)[D").unwrap().to_return_type(), JType::array("[D"));
    assert_eq!(JMethodType::parse("(I[Ljava/lang/Object;D)Ljava/lang/String;").unwrap().to_return_type(),
               JType::object("Ljava/lang/String;"));
}
