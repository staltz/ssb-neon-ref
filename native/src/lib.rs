#[macro_use]
extern crate neon;
extern crate ssb_multiformats;

use neon::prelude::*;
use ssb_multiformats::multihash::Multihash;

fn is_blob(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let input = {
    cx.argument::<JsString>(0).and_then(|v| {
      if v.is_a::<JsString>() {
        Ok(v)
      } else {
        cx.throw_error("expected string as the argument to `isBlob`")
      }
    })
  }?.value();

  let is_ok = Multihash::from_legacy(input.as_bytes()).is_ok();

  Ok(cx.boolean(is_ok))
}

register_module!(mut cx, { cx.export_function("isBlob", is_blob) });
