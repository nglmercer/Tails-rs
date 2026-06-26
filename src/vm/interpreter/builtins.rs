use std::collections::HashMap;
use crate::objects::Value;
use super::{Interpreter, HeapValue, JsObject};

impl Interpreter {
    pub(super) fn init_builtins(&mut self) {
        // Global functions
        self.globals.insert("parseInt".into(), Value::NativeFunction(10));
        self.globals.insert("parseFloat".into(), Value::NativeFunction(11));
        self.globals.insert("isNaN".into(), Value::NativeFunction(12));
        self.globals.insert("isFinite".into(), Value::NativeFunction(13));

        // Timer stubs
        self.globals.insert("setTimeout".into(), Value::NativeFunction(14));
        self.globals.insert("setInterval".into(), Value::NativeFunction(15));
        self.globals.insert("clearTimeout".into(), Value::NativeFunction(16));
        self.globals.insert("clearInterval".into(), Value::NativeFunction(17));

        // console object
        let mut console_props = HashMap::new();
        console_props.insert("log".into(), Value::NativeFunction(0));
        console_props.insert("warn".into(), Value::NativeFunction(1));
        console_props.insert("error".into(), Value::NativeFunction(2));
        console_props.insert("info".into(), Value::NativeFunction(3));
        let console_obj_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: console_props, prototype: None }));
        self.globals.insert("console".into(), Value::Object(console_obj_idx));

        // Object
        let mut object_props = HashMap::new();
        object_props.insert("keys".into(), Value::NativeFunction(4));
        object_props.insert("values".into(), Value::NativeFunction(5));
        object_props.insert("entries".into(), Value::NativeFunction(6));
        object_props.insert("assign".into(), Value::NativeFunction(7));
        object_props.insert("defineProperty".into(), Value::NativeFunction(99));
        object_props.insert("getOwnPropertyDescriptor".into(), Value::NativeFunction(100));
        object_props.insert("freeze".into(), Value::NativeFunction(101));
        let object_obj_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: object_props, prototype: None }));
        self.globals.insert("Object".into(), Value::Object(object_obj_idx));

        // Proxy
        self.globals.insert("Proxy".into(), Value::NativeFunction(85));

        // Reflect
        let mut reflect_props = HashMap::new();
        reflect_props.insert("get".into(), Value::NativeFunction(86));
        reflect_props.insert("set".into(), Value::NativeFunction(87));
        reflect_props.insert("has".into(), Value::NativeFunction(88));
        reflect_props.insert("deleteProperty".into(), Value::NativeFunction(89));
        reflect_props.insert("apply".into(), Value::NativeFunction(90));
        reflect_props.insert("construct".into(), Value::NativeFunction(91));
        reflect_props.insert("ownKeys".into(), Value::NativeFunction(92));
        reflect_props.insert("getOwnPropertyDescriptor".into(), Value::NativeFunction(93));
        reflect_props.insert("defineProperty".into(), Value::NativeFunction(94));
        reflect_props.insert("getPrototypeOf".into(), Value::NativeFunction(95));
        reflect_props.insert("setPrototypeOf".into(), Value::NativeFunction(96));
        reflect_props.insert("isExtensible".into(), Value::NativeFunction(97));
        reflect_props.insert("preventExtensions".into(), Value::NativeFunction(98));
        let reflect_obj_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: reflect_props, prototype: None }));
        self.globals.insert("Reflect".into(), Value::Object(reflect_obj_idx));

        // JSON
        let mut json_props = HashMap::new();
        json_props.insert("parse".into(), Value::NativeFunction(8));
        json_props.insert("stringify".into(), Value::NativeFunction(9));
        let json_obj_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: json_props, prototype: None }));
        self.globals.insert("JSON".into(), Value::Object(json_obj_idx));

        // Math
        let mut math_props = HashMap::new();
        math_props.insert("PI".into(), Value::Float(std::f64::consts::PI));
        math_props.insert("E".into(), Value::Float(std::f64::consts::E));
        math_props.insert("abs".into(), Value::NativeFunction(18));
        math_props.insert("floor".into(), Value::NativeFunction(19));
        math_props.insert("ceil".into(), Value::NativeFunction(20));
        math_props.insert("round".into(), Value::NativeFunction(21));
        math_props.insert("min".into(), Value::NativeFunction(22));
        math_props.insert("max".into(), Value::NativeFunction(23));
        math_props.insert("random".into(), Value::NativeFunction(24));
        math_props.insert("pow".into(), Value::NativeFunction(25));
        math_props.insert("sqrt".into(), Value::NativeFunction(26));
        math_props.insert("log".into(), Value::NativeFunction(27));
        math_props.insert("sin".into(), Value::NativeFunction(28));
        math_props.insert("cos".into(), Value::NativeFunction(29));
        math_props.insert("tan".into(), Value::NativeFunction(30));
        let math_obj_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: math_props, prototype: None }));
        self.globals.insert("Math".into(), Value::Object(math_obj_idx));

        // Number constructor
        let mut number_props = HashMap::new();
        number_props.insert("isFinite".into(), Value::NativeFunction(13));
        number_props.insert("isNaN".into(), Value::NativeFunction(12));
        number_props.insert("parseFloat".into(), Value::NativeFunction(11));
        number_props.insert("parseInt".into(), Value::NativeFunction(10));
        let number_obj_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: number_props, prototype: None }));
        self.globals.insert("Number".into(), Value::Object(number_obj_idx));

        // Promise constructor and prototype
        let mut promise_proto_props = HashMap::new();
        promise_proto_props.insert("then".into(), Value::NativeFunction(78));
        promise_proto_props.insert("catch".into(), Value::NativeFunction(79));
        promise_proto_props.insert("finally".into(), Value::NativeFunction(80));
        let promise_proto_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: promise_proto_props, prototype: None }));

        let mut promise_ctor_props = HashMap::new();
        promise_ctor_props.insert("prototype".into(), Value::Object(promise_proto_idx));
        promise_ctor_props.insert("resolve".into(), Value::NativeFunction(81));
        promise_ctor_props.insert("reject".into(), Value::NativeFunction(82));
        promise_ctor_props.insert("all".into(), Value::NativeFunction(83));
        promise_ctor_props.insert("race".into(), Value::NativeFunction(84));
        self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: promise_ctor_props, prototype: None }));
        self.globals.insert("Promise".into(), Value::NativeFunction(77));

        // Error constructor
        let error_proto_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject::new()));
        let mut error_ctor_props = HashMap::new();
        error_ctor_props.insert("prototype".into(), Value::Object(error_proto_idx));
        self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: error_ctor_props, prototype: None }));
        self.globals.insert("Error".into(), Value::NativeFunction(72));

        // TypeError constructor
        let mut type_error_proto_props = HashMap::new();
        type_error_proto_props.insert("name".into(), Value::String("TypeError".into()));
        let type_error_proto_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: type_error_proto_props, prototype: Some(error_proto_idx) }));
        let mut type_error_ctor_props = HashMap::new();
        type_error_ctor_props.insert("prototype".into(), Value::Object(type_error_proto_idx));
        self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: type_error_ctor_props, prototype: None }));
        self.globals.insert("TypeError".into(), Value::NativeFunction(73));

        // ReferenceError constructor
        let mut ref_error_proto_props = HashMap::new();
        ref_error_proto_props.insert("name".into(), Value::String("ReferenceError".into()));
        let ref_error_proto_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: ref_error_proto_props, prototype: Some(error_proto_idx) }));
        let mut ref_error_ctor_props = HashMap::new();
        ref_error_ctor_props.insert("prototype".into(), Value::Object(ref_error_proto_idx));
        self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: ref_error_ctor_props, prototype: None }));
        self.globals.insert("ReferenceError".into(), Value::NativeFunction(74));

        // SyntaxError constructor
        let mut syntax_error_proto_props = HashMap::new();
        syntax_error_proto_props.insert("name".into(), Value::String("SyntaxError".into()));
        let syntax_error_proto_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: syntax_error_proto_props, prototype: Some(error_proto_idx) }));
        let mut syntax_error_ctor_props = HashMap::new();
        syntax_error_ctor_props.insert("prototype".into(), Value::Object(syntax_error_proto_idx));
        self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: syntax_error_ctor_props, prototype: None }));
        self.globals.insert("SyntaxError".into(), Value::NativeFunction(75));

        // RangeError constructor
        let mut range_error_proto_props = HashMap::new();
        range_error_proto_props.insert("name".into(), Value::String("RangeError".into()));
        let range_error_proto_idx = self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: range_error_proto_props, prototype: Some(error_proto_idx) }));
        let mut range_error_ctor_props = HashMap::new();
        range_error_ctor_props.insert("prototype".into(), Value::Object(range_error_proto_idx));
        self.gc.allocate(&mut self.heap, HeapValue::Object(JsObject { properties: range_error_ctor_props, prototype: None }));
        self.globals.insert("RangeError".into(), Value::NativeFunction(76));
    }
}
