//! `flash.events.Event` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;

pub use crate::avm2::object::event_allocator;
use crate::avm2::parameters::ParametersExt;

pub fn init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let this = this.unwrap();
    let mut evt = this.as_event_mut(activation.context.gc_context).unwrap();
    evt.set_event_type(args.get_string(activation, 0)?);
    evt.set_bubbles(args.get_bool(1));
    evt.set_cancelable(args.get_bool(2));
    Ok(Value::Undefined)
}

/// Implements `bubbles` property's getter
pub fn get_bubbles<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.is_bubbling().into());
    }

    Ok(Value::Undefined)
}

/// Implements `cancelable` property's getter
pub fn get_cancelable<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.is_cancelable().into());
    }

    Ok(Value::Undefined)
}

/// Implements `type` property's getter
pub fn get_type<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.event_type().into());
    }

    Ok(Value::Undefined)
}

/// Implements `target` property's getter
pub fn get_target<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.target().map(|o| o.into()).unwrap_or(Value::Null));
    }

    Ok(Value::Undefined)
}

/// Implements `currentTarget` property's getter
pub fn get_current_target<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt
            .current_target()
            .map(|o| o.into())
            .unwrap_or(Value::Null));
    }

    Ok(Value::Undefined)
}

/// Implements `eventPhase` property's getter
pub fn get_event_phase<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        let event_phase = evt.phase() as u32;
        return Ok(event_phase.into());
    }

    Ok(Value::Undefined)
}

/// Implements `isDefaultPrevented`
pub fn is_default_prevented<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(evt) = this.unwrap().as_event() {
        return Ok(evt.is_cancelled().into());
    }

    Ok(Value::Undefined)
}

/// Implements `preventDefault`
pub fn prevent_default<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut evt) = this.unwrap().as_event_mut(activation.context.gc_context) {
        evt.cancel();
    }

    Ok(Value::Undefined)
}

/// Implements `stopPropagation`
pub fn stop_propagation<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut evt) = this.unwrap().as_event_mut(activation.context.gc_context) {
        evt.stop_propagation();
    }

    Ok(Value::Undefined)
}

/// Implements `stopImmediatePropagation`
pub fn stop_immediate_propagation<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut evt) = this.unwrap().as_event_mut(activation.context.gc_context) {
        evt.stop_immediate_propagation();
    }

    Ok(Value::Undefined)
}
