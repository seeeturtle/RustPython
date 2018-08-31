use super::obj::objdict;
use super::obj::objtype;
use super::pyobject::{
    AttributeProtocol, IdProtocol, PyContext, PyFuncArgs, PyObject, PyObjectKind, PyObjectRef,
    PyResult, TypeProtocol,
};
use super::vm::VirtualMachine;

pub fn new_instance(vm: &mut VirtualMachine, mut args: PyFuncArgs) -> PyResult {
    // more or less __new__ operator
    let type_ref = args.shift();
    let dict = vm.new_dict();
    let obj = PyObject::new(PyObjectKind::Instance { dict: dict }, type_ref.clone());
    Ok(obj)
}

pub fn call(vm: &mut VirtualMachine, mut args: PyFuncArgs) -> PyResult {
    let instance = args.shift();
    let function = objtype::get_attribute(vm, instance, &String::from("__call__"))?;
    vm.invoke(function, args)
}

pub fn create_object(type_type: PyObjectRef, object_type: PyObjectRef, dict_type: PyObjectRef) {
    (*object_type.borrow_mut()).kind = PyObjectKind::Class {
        name: String::from("object"),
        dict: objdict::new(dict_type),
        mro: vec![],
    };
    (*object_type.borrow_mut()).typ = Some(type_type.clone());
}

fn obj_str(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(obj, Some(vm.ctx.object()))]);
    let type_name = objtype::get_type_name(&obj.typ());
    let address = obj.get_id();
    Ok(vm.new_str(format!("<{} object at 0x{:x}>", type_name, address)))
}

pub fn init(context: &PyContext) {
    let ref object = context.object;
    object.set_attr("__new__", context.new_rustfunc(new_instance));
    object.set_attr("__dict__", context.new_member_descriptor(object_dict));
    object.set_attr("__str__", context.new_rustfunc(obj_str));
}

fn object_dict(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    match args.args[0].borrow().kind {
        PyObjectKind::Class { ref dict, .. } => Ok(dict.clone()),
        PyObjectKind::Instance { ref dict, .. } => Ok(dict.clone()),
        _ => Err(vm.new_type_error("TypeError: no dictionary.".to_string())),
    }
}
