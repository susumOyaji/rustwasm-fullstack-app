var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/worker/shim.mjs
import Ze from "./cbbff4226a7d01d9a7ec69ac8f425196f5b3bdf2-index.wasm";
import { WorkerEntrypoint as tn } from "cloudflare:workers";
var M = Object.defineProperty;
var U = /* @__PURE__ */ __name((t, e) => {
  for (var n in e) M(t, n, { get: e[n], enumerable: true });
}, "U");
var d = {};
U(d, { IntoUnderlyingByteSource: /* @__PURE__ */ __name(() => j, "IntoUnderlyingByteSource"), IntoUnderlyingSink: /* @__PURE__ */ __name(() => I, "IntoUnderlyingSink"), IntoUnderlyingSource: /* @__PURE__ */ __name(() => E, "IntoUnderlyingSource"), MinifyConfig: /* @__PURE__ */ __name(() => p, "MinifyConfig"), PolishConfig: /* @__PURE__ */ __name(() => P, "PolishConfig"), R2Range: /* @__PURE__ */ __name(() => O, "R2Range"), RequestRedirect: /* @__PURE__ */ __name(() => V, "RequestRedirect"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(() => Q, "__wbg_String_8f0eb39a4a4c2f66"), __wbg_body_0b8fd1fe671660df: /* @__PURE__ */ __name(() => Y, "__wbg_body_0b8fd1fe671660df"), __wbg_buffer_09165b52af8c5237: /* @__PURE__ */ __name(() => Z, "__wbg_buffer_09165b52af8c5237"), __wbg_buffer_609cc3eee51ed158: /* @__PURE__ */ __name(() => tt, "__wbg_buffer_609cc3eee51ed158"), __wbg_byobRequest_77d9adf63337edfb: /* @__PURE__ */ __name(() => et, "__wbg_byobRequest_77d9adf63337edfb"), __wbg_byteLength_e674b853d9c77e1d: /* @__PURE__ */ __name(() => nt, "__wbg_byteLength_e674b853d9c77e1d"), __wbg_byteOffset_fd862df290ef848d: /* @__PURE__ */ __name(() => rt, "__wbg_byteOffset_fd862df290ef848d"), __wbg_call_672a4d21634d4a24: /* @__PURE__ */ __name(() => _t, "__wbg_call_672a4d21634d4a24"), __wbg_call_7cccdd69e0791ae2: /* @__PURE__ */ __name(() => ot, "__wbg_call_7cccdd69e0791ae2"), __wbg_call_833bed5770ea2041: /* @__PURE__ */ __name(() => ct, "__wbg_call_833bed5770ea2041"), __wbg_call_b8adc8b1d0a0d8eb: /* @__PURE__ */ __name(() => it, "__wbg_call_b8adc8b1d0a0d8eb"), __wbg_cancel_8a308660caa6cadf: /* @__PURE__ */ __name(() => st, "__wbg_cancel_8a308660caa6cadf"), __wbg_catch_a6e601879b2610e9: /* @__PURE__ */ __name(() => ut, "__wbg_catch_a6e601879b2610e9"), __wbg_cause_9940c4e8dfcd5129: /* @__PURE__ */ __name(() => ft, "__wbg_cause_9940c4e8dfcd5129"), __wbg_cf_123509d53a2ea003: /* @__PURE__ */ __name(() => at, "__wbg_cf_123509d53a2ea003"), __wbg_cf_abc51304c8a6868c: /* @__PURE__ */ __name(() => gt, "__wbg_cf_abc51304c8a6868c"), __wbg_close_304cc1fef3466669: /* @__PURE__ */ __name(() => bt, "__wbg_close_304cc1fef3466669"), __wbg_close_5ce03e29be453811: /* @__PURE__ */ __name(() => dt, "__wbg_close_5ce03e29be453811"), __wbg_constructor_9fd96f589d65d4e5: /* @__PURE__ */ __name(() => wt, "__wbg_constructor_9fd96f589d65d4e5"), __wbg_enqueue_bb16ba72f537dc9e: /* @__PURE__ */ __name(() => lt, "__wbg_enqueue_bb16ba72f537dc9e"), __wbg_error_524f506f44df1645: /* @__PURE__ */ __name(() => pt, "__wbg_error_524f506f44df1645"), __wbg_fetch_07cd86dd296a5a63: /* @__PURE__ */ __name(() => xt, "__wbg_fetch_07cd86dd296a5a63"), __wbg_fetch_79398949f1862502: /* @__PURE__ */ __name(() => yt, "__wbg_fetch_79398949f1862502"), __wbg_getReader_48e00749fe3f6089: /* @__PURE__ */ __name(() => ht, "__wbg_getReader_48e00749fe3f6089"), __wbg_get_123509460060ab98: /* @__PURE__ */ __name(() => mt, "__wbg_get_123509460060ab98"), __wbg_get_67b2ba62fc30de12: /* @__PURE__ */ __name(() => Rt, "__wbg_get_67b2ba62fc30de12"), __wbg_getdone_d47073731acd3e74: /* @__PURE__ */ __name(() => Ft, "__wbg_getdone_d47073731acd3e74"), __wbg_getvalue_009dcd63692bee1f: /* @__PURE__ */ __name(() => St, "__wbg_getvalue_009dcd63692bee1f"), __wbg_getwithrefkey_1dc361bd10053bfe: /* @__PURE__ */ __name(() => jt, "__wbg_getwithrefkey_1dc361bd10053bfe"), __wbg_getwithrefkey_6550b2c093d2eb18: /* @__PURE__ */ __name(() => It, "__wbg_getwithrefkey_6550b2c093d2eb18"), __wbg_headers_7852a8ea641c1379: /* @__PURE__ */ __name(() => Et, "__wbg_headers_7852a8ea641c1379"), __wbg_headers_9cb51cfd2ac780a4: /* @__PURE__ */ __name(() => Ot, "__wbg_headers_9cb51cfd2ac780a4"), __wbg_instanceof_ArrayBuffer_e14585432e3737fc: /* @__PURE__ */ __name(() => kt, "__wbg_instanceof_ArrayBuffer_e14585432e3737fc"), __wbg_instanceof_Error_4d54113b22d20306: /* @__PURE__ */ __name(() => zt, "__wbg_instanceof_Error_4d54113b22d20306"), __wbg_instanceof_ReadableStream_87eac785b90f3611: /* @__PURE__ */ __name(() => At, "__wbg_instanceof_ReadableStream_87eac785b90f3611"), __wbg_instanceof_Response_f2cc20d9f7dfd644: /* @__PURE__ */ __name(() => Tt, "__wbg_instanceof_Response_f2cc20d9f7dfd644"), __wbg_instanceof_Uint8Array_17156bcf118086a9: /* @__PURE__ */ __name(() => qt, "__wbg_instanceof_Uint8Array_17156bcf118086a9"), __wbg_json_a00f187c0be01957: /* @__PURE__ */ __name(() => Lt, "__wbg_json_a00f187c0be01957"), __wbg_length_a446193dc22c12f8: /* @__PURE__ */ __name(() => Mt, "__wbg_length_a446193dc22c12f8"), __wbg_log_c222819a41e063d3: /* @__PURE__ */ __name(() => Ut, "__wbg_log_c222819a41e063d3"), __wbg_method_3dcc854b644c5a56: /* @__PURE__ */ __name(() => Dt, "__wbg_method_3dcc854b644c5a56"), __wbg_minifyconfig_new: /* @__PURE__ */ __name(() => Ct, "__wbg_minifyconfig_new"), __wbg_name_16617c8e9d4188ac: /* @__PURE__ */ __name(() => Wt, "__wbg_name_16617c8e9d4188ac"), __wbg_new_018dcc2d6c8c2f6a: /* @__PURE__ */ __name(() => Bt, "__wbg_new_018dcc2d6c8c2f6a"), __wbg_new_23a2665fac83c611: /* @__PURE__ */ __name(() => Nt, "__wbg_new_23a2665fac83c611"), __wbg_new_405e22f390576ce2: /* @__PURE__ */ __name(() => $t, "__wbg_new_405e22f390576ce2"), __wbg_new_5e0be73521bc8c17: /* @__PURE__ */ __name(() => Pt, "__wbg_new_5e0be73521bc8c17"), __wbg_new_78feb108b6472713: /* @__PURE__ */ __name(() => Vt, "__wbg_new_78feb108b6472713"), __wbg_new_a12002a7f91c75be: /* @__PURE__ */ __name(() => vt, "__wbg_new_a12002a7f91c75be"), __wbg_new_c68d7209be747379: /* @__PURE__ */ __name(() => Gt, "__wbg_new_c68d7209be747379"), __wbg_newnoargs_105ed471475aaf50: /* @__PURE__ */ __name(() => Ht, "__wbg_newnoargs_105ed471475aaf50"), __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a: /* @__PURE__ */ __name(() => Jt, "__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a"), __wbg_newwithlength_a381634e90c276d4: /* @__PURE__ */ __name(() => Xt, "__wbg_newwithlength_a381634e90c276d4"), __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1: /* @__PURE__ */ __name(() => Kt, "__wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1"), __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e: /* @__PURE__ */ __name(() => Qt, "__wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e"), __wbg_newwithoptstrandinit_615a266ef226c260: /* @__PURE__ */ __name(() => Yt, "__wbg_newwithoptstrandinit_615a266ef226c260"), __wbg_newwithstrandinit_06c535e0a867c635: /* @__PURE__ */ __name(() => Zt, "__wbg_newwithstrandinit_06c535e0a867c635"), __wbg_queueMicrotask_97d92b4fcc8a61c5: /* @__PURE__ */ __name(() => te, "__wbg_queueMicrotask_97d92b4fcc8a61c5"), __wbg_queueMicrotask_d3219def82552485: /* @__PURE__ */ __name(() => ee, "__wbg_queueMicrotask_d3219def82552485"), __wbg_read_a2434af1186cb56c: /* @__PURE__ */ __name(() => ne, "__wbg_read_a2434af1186cb56c"), __wbg_releaseLock_091899af97991d2e: /* @__PURE__ */ __name(() => re, "__wbg_releaseLock_091899af97991d2e"), __wbg_resolve_4851785c9c5f573d: /* @__PURE__ */ __name(() => _e, "__wbg_resolve_4851785c9c5f573d"), __wbg_respond_1f279fa9f8edcb1c: /* @__PURE__ */ __name(() => oe, "__wbg_respond_1f279fa9f8edcb1c"), __wbg_set_11cd83f45504cedf: /* @__PURE__ */ __name(() => ce, "__wbg_set_11cd83f45504cedf"), __wbg_set_37837023f3d740e8: /* @__PURE__ */ __name(() => ie, "__wbg_set_37837023f3d740e8"), __wbg_set_3807d5f0bfc24aa7: /* @__PURE__ */ __name(() => se, "__wbg_set_3807d5f0bfc24aa7"), __wbg_set_3f1d0b984ed272ed: /* @__PURE__ */ __name(() => ue, "__wbg_set_3f1d0b984ed272ed"), __wbg_set_65595bdd868b3009: /* @__PURE__ */ __name(() => fe, "__wbg_set_65595bdd868b3009"), __wbg_set_8fc6bf8a5b1071d1: /* @__PURE__ */ __name(() => ae, "__wbg_set_8fc6bf8a5b1071d1"), __wbg_set_bb8cecf6a62b9f46: /* @__PURE__ */ __name(() => ge, "__wbg_set_bb8cecf6a62b9f46"), __wbg_set_wasm: /* @__PURE__ */ __name(() => k, "__wbg_set_wasm"), __wbg_setbody_5923b78a95eedf29: /* @__PURE__ */ __name(() => be, "__wbg_setbody_5923b78a95eedf29"), __wbg_setheaders_3b47c898e8de6d44: /* @__PURE__ */ __name(() => de, "__wbg_setheaders_3b47c898e8de6d44"), __wbg_setheaders_834c0bdb6a8949ad: /* @__PURE__ */ __name(() => we, "__wbg_setheaders_834c0bdb6a8949ad"), __wbg_setmethod_3c5280fe5d890842: /* @__PURE__ */ __name(() => le, "__wbg_setmethod_3c5280fe5d890842"), __wbg_setredirect_40e6a7f717a2f86a: /* @__PURE__ */ __name(() => pe, "__wbg_setredirect_40e6a7f717a2f86a"), __wbg_setsignal_75b21ef3a81de905: /* @__PURE__ */ __name(() => xe, "__wbg_setsignal_75b21ef3a81de905"), __wbg_setstatus_51b4fc011091cbb3: /* @__PURE__ */ __name(() => ye, "__wbg_setstatus_51b4fc011091cbb3"), __wbg_static_accessor_GLOBAL_88a902d13a557d07: /* @__PURE__ */ __name(() => he, "__wbg_static_accessor_GLOBAL_88a902d13a557d07"), __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0: /* @__PURE__ */ __name(() => me, "__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0"), __wbg_static_accessor_SELF_37c5d418e4bf5819: /* @__PURE__ */ __name(() => Re, "__wbg_static_accessor_SELF_37c5d418e4bf5819"), __wbg_static_accessor_WINDOW_5de37043a91a9c40: /* @__PURE__ */ __name(() => Fe, "__wbg_static_accessor_WINDOW_5de37043a91a9c40"), __wbg_status_f6360336ca686bf0: /* @__PURE__ */ __name(() => Se, "__wbg_status_f6360336ca686bf0"), __wbg_stringify_f7ed6987935b4a24: /* @__PURE__ */ __name(() => je, "__wbg_stringify_f7ed6987935b4a24"), __wbg_then_44b73946d2fb3e7d: /* @__PURE__ */ __name(() => Ie, "__wbg_then_44b73946d2fb3e7d"), __wbg_then_48b406749878a531: /* @__PURE__ */ __name(() => Ee, "__wbg_then_48b406749878a531"), __wbg_toString_c813bbd34d063839: /* @__PURE__ */ __name(() => Oe, "__wbg_toString_c813bbd34d063839"), __wbg_url_8f9653b899456042: /* @__PURE__ */ __name(() => ke, "__wbg_url_8f9653b899456042"), __wbg_view_fd8a56e8983f448d: /* @__PURE__ */ __name(() => ze, "__wbg_view_fd8a56e8983f448d"), __wbg_webSocket_4a16d2250705e19d: /* @__PURE__ */ __name(() => Ae, "__wbg_webSocket_4a16d2250705e19d"), __wbindgen_bigint_from_i64: /* @__PURE__ */ __name(() => Te, "__wbindgen_bigint_from_i64"), __wbindgen_bigint_from_u64: /* @__PURE__ */ __name(() => qe, "__wbindgen_bigint_from_u64"), __wbindgen_boolean_get: /* @__PURE__ */ __name(() => Le, "__wbindgen_boolean_get"), __wbindgen_cb_drop: /* @__PURE__ */ __name(() => Me, "__wbindgen_cb_drop"), __wbindgen_closure_wrapper1830: /* @__PURE__ */ __name(() => Ue, "__wbindgen_closure_wrapper1830"), __wbindgen_debug_string: /* @__PURE__ */ __name(() => De, "__wbindgen_debug_string"), __wbindgen_error_new: /* @__PURE__ */ __name(() => Ce, "__wbindgen_error_new"), __wbindgen_in: /* @__PURE__ */ __name(() => We, "__wbindgen_in"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(() => Be, "__wbindgen_init_externref_table"), __wbindgen_is_function: /* @__PURE__ */ __name(() => Ne, "__wbindgen_is_function"), __wbindgen_is_null: /* @__PURE__ */ __name(() => $e, "__wbindgen_is_null"), __wbindgen_is_object: /* @__PURE__ */ __name(() => Pe, "__wbindgen_is_object"), __wbindgen_is_string: /* @__PURE__ */ __name(() => Ve, "__wbindgen_is_string"), __wbindgen_is_undefined: /* @__PURE__ */ __name(() => ve, "__wbindgen_is_undefined"), __wbindgen_jsval_loose_eq: /* @__PURE__ */ __name(() => Ge, "__wbindgen_jsval_loose_eq"), __wbindgen_memory: /* @__PURE__ */ __name(() => He, "__wbindgen_memory"), __wbindgen_number_get: /* @__PURE__ */ __name(() => Je, "__wbindgen_number_get"), __wbindgen_number_new: /* @__PURE__ */ __name(() => Xe, "__wbindgen_number_new"), __wbindgen_string_get: /* @__PURE__ */ __name(() => Ke, "__wbindgen_string_get"), __wbindgen_string_new: /* @__PURE__ */ __name(() => Qe, "__wbindgen_string_new"), __wbindgen_throw: /* @__PURE__ */ __name(() => Ye, "__wbindgen_throw"), fetch: /* @__PURE__ */ __name(() => z, "fetch") });
var _;
function k(t) {
  _ = t;
}
__name(k, "k");
var w = 0;
var h = null;
function m() {
  return (h === null || h.byteLength === 0) && (h = new Uint8Array(_.memory.buffer)), h;
}
__name(m, "m");
var D = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var R = new D("utf-8");
var C = typeof R.encodeInto == "function" ? function(t, e) {
  return R.encodeInto(t, e);
} : function(t, e) {
  let n = R.encode(t);
  return e.set(n), { read: t.length, written: n.length };
};
function x(t, e, n) {
  if (n === void 0) {
    let f = R.encode(t), y = e(f.length, 1) >>> 0;
    return m().subarray(y, y + f.length).set(f), w = f.length, y;
  }
  let r = t.length, o = e(r, 1) >>> 0, u = m(), i = 0;
  for (; i < r; i++) {
    let f = t.charCodeAt(i);
    if (f > 127) break;
    u[o + i] = f;
  }
  if (i !== r) {
    i !== 0 && (t = t.slice(i)), o = n(o, r, r = i + t.length * 3, 1) >>> 0;
    let f = m().subarray(o + i, o + r);
    i += C(t, f).written, o = n(o, r, i, 1) >>> 0;
  }
  return w = i, o;
}
__name(x, "x");
var l = null;
function a() {
  return (l === null || l.buffer.detached === true || l.buffer.detached === void 0 && l.buffer !== _.memory.buffer) && (l = new DataView(_.memory.buffer)), l;
}
__name(a, "a");
function s(t) {
  return t == null;
}
__name(s, "s");
function b(t) {
  let e = _.__externref_table_alloc();
  return _.__wbindgen_export_3.set(e, t), e;
}
__name(b, "b");
function c(t, e) {
  try {
    return t.apply(this, e);
  } catch (n) {
    let r = b(n);
    _.__wbindgen_exn_store(r);
  }
}
__name(c, "c");
var W = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var q = new W("utf-8", { ignoreBOM: true, fatal: true });
q.decode();
function g(t, e) {
  return t = t >>> 0, q.decode(m().subarray(t, t + e));
}
__name(g, "g");
var A = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => {
  _.__wbindgen_export_5.get(t.dtor)(t.a, t.b);
});
function B(t, e, n, r) {
  let o = { a: t, b: e, cnt: 1, dtor: n }, u = /* @__PURE__ */ __name((...i) => {
    o.cnt++;
    let f = o.a;
    o.a = 0;
    try {
      return r(f, o.b, ...i);
    } finally {
      --o.cnt === 0 ? (_.__wbindgen_export_5.get(o.dtor)(f, o.b), A.unregister(o)) : o.a = f;
    }
  }, "u");
  return u.original = o, A.register(u, o, o), u;
}
__name(B, "B");
function S(t) {
  let e = typeof t;
  if (e == "number" || e == "boolean" || t == null) return `${t}`;
  if (e == "string") return `"${t}"`;
  if (e == "symbol") {
    let o = t.description;
    return o == null ? "Symbol" : `Symbol(${o})`;
  }
  if (e == "function") {
    let o = t.name;
    return typeof o == "string" && o.length > 0 ? `Function(${o})` : "Function";
  }
  if (Array.isArray(t)) {
    let o = t.length, u = "[";
    o > 0 && (u += S(t[0]));
    for (let i = 1; i < o; i++) u += ", " + S(t[i]);
    return u += "]", u;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(t)), r;
  if (n && n.length > 1) r = n[1];
  else return toString.call(t);
  if (r == "Object") try {
    return "Object(" + JSON.stringify(t) + ")";
  } catch {
    return "Object";
  }
  return t instanceof Error ? `${t.name}: ${t.message}
${t.stack}` : r;
}
__name(S, "S");
function z(t, e, n) {
  return _.fetch(t, e, n);
}
__name(z, "z");
function N(t, e, n) {
  _.closure365_externref_shim(t, e, n);
}
__name(N, "N");
function $(t, e, n, r) {
  _.closure408_externref_shim(t, e, n, r);
}
__name($, "$");
var P = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var V = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var v = ["bytes"];
var G = ["follow", "error", "manual"];
var H = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_intounderlyingbytesource_free(t >>> 0, 1));
var j = class {
  static {
    __name(this, "j");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, H.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingbytesource_free(e, 0);
  }
  get type() {
    let e = _.intounderlyingbytesource_type(this.__wbg_ptr);
    return v[e];
  }
  get autoAllocateChunkSize() {
    return _.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(e) {
    _.intounderlyingbytesource_start(this.__wbg_ptr, e);
  }
  pull(e) {
    return _.intounderlyingbytesource_pull(this.__wbg_ptr, e);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    _.intounderlyingbytesource_cancel(e);
  }
};
var J = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_intounderlyingsink_free(t >>> 0, 1));
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, J.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingsink_free(e, 0);
  }
  write(e) {
    return _.intounderlyingsink_write(this.__wbg_ptr, e);
  }
  close() {
    let e = this.__destroy_into_raw();
    return _.intounderlyingsink_close(e);
  }
  abort(e) {
    let n = this.__destroy_into_raw();
    return _.intounderlyingsink_abort(n, e);
  }
};
var X = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_intounderlyingsource_free(t >>> 0, 1));
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, X.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingsource_free(e, 0);
  }
  pull(e) {
    return _.intounderlyingsource_pull(this.__wbg_ptr, e);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    _.intounderlyingsource_cancel(e);
  }
};
var T = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_minifyconfig_free(t >>> 0, 1));
var p = class {
  static {
    __name(this, "p");
  }
  static __wrap(e) {
    e = e >>> 0;
    let n = Object.create(p.prototype);
    return n.__wbg_ptr = e, T.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, T.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_minifyconfig_free(e, 0);
  }
  get js() {
    return _.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(e) {
    _.__wbg_set_minifyconfig_js(this.__wbg_ptr, e);
  }
  get html() {
    return _.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(e) {
    _.__wbg_set_minifyconfig_html(this.__wbg_ptr, e);
  }
  get css() {
    return _.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(e) {
    _.__wbg_set_minifyconfig_css(this.__wbg_ptr, e);
  }
};
var K = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_r2range_free(t >>> 0, 1));
var O = class {
  static {
    __name(this, "O");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, K.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_r2range_free(e, 0);
  }
  get offset() {
    let e = _.__wbg_get_r2range_offset(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set offset(e) {
    _.__wbg_set_r2range_offset(this.__wbg_ptr, !s(e), s(e) ? 0 : e);
  }
  get length() {
    let e = _.__wbg_get_r2range_length(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set length(e) {
    _.__wbg_set_r2range_length(this.__wbg_ptr, !s(e), s(e) ? 0 : e);
  }
  get suffix() {
    let e = _.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set suffix(e) {
    _.__wbg_set_r2range_suffix(this.__wbg_ptr, !s(e), s(e) ? 0 : e);
  }
};
function Q(t, e) {
  let n = String(e), r = x(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(Q, "Q");
function Y(t) {
  let e = t.body;
  return s(e) ? 0 : b(e);
}
__name(Y, "Y");
function Z(t) {
  return t.buffer;
}
__name(Z, "Z");
function tt(t) {
  return t.buffer;
}
__name(tt, "tt");
function et(t) {
  let e = t.byobRequest;
  return s(e) ? 0 : b(e);
}
__name(et, "et");
function nt(t) {
  return t.byteLength;
}
__name(nt, "nt");
function rt(t) {
  return t.byteOffset;
}
__name(rt, "rt");
function _t() {
  return c(function(t, e) {
    return t.call(e);
  }, arguments);
}
__name(_t, "_t");
function ot() {
  return c(function(t, e, n) {
    return t.call(e, n);
  }, arguments);
}
__name(ot, "ot");
function ct() {
  return c(function(t, e, n, r) {
    return t.call(e, n, r);
  }, arguments);
}
__name(ct, "ct");
function it() {
  return c(function(t, e, n, r, o) {
    return t.call(e, n, r, o);
  }, arguments);
}
__name(it, "it");
function st(t) {
  return t.cancel();
}
__name(st, "st");
function ut(t, e) {
  return t.catch(e);
}
__name(ut, "ut");
function ft(t) {
  return t.cause;
}
__name(ft, "ft");
function at() {
  return c(function(t) {
    let e = t.cf;
    return s(e) ? 0 : b(e);
  }, arguments);
}
__name(at, "at");
function gt() {
  return c(function(t) {
    let e = t.cf;
    return s(e) ? 0 : b(e);
  }, arguments);
}
__name(gt, "gt");
function bt() {
  return c(function(t) {
    t.close();
  }, arguments);
}
__name(bt, "bt");
function dt() {
  return c(function(t) {
    t.close();
  }, arguments);
}
__name(dt, "dt");
function wt(t) {
  return t.constructor;
}
__name(wt, "wt");
function lt() {
  return c(function(t, e) {
    t.enqueue(e);
  }, arguments);
}
__name(lt, "lt");
function pt(t) {
  console.error(t);
}
__name(pt, "pt");
function xt(t, e, n) {
  return t.fetch(e, n);
}
__name(xt, "xt");
function yt(t, e, n, r) {
  return t.fetch(g(e, n), r);
}
__name(yt, "yt");
function ht() {
  return c(function(t) {
    return t.getReader();
  }, arguments);
}
__name(ht, "ht");
function mt() {
  return c(function(t, e, n, r) {
    let o = e.get(g(n, r));
    var u = s(o) ? 0 : x(o, _.__wbindgen_malloc, _.__wbindgen_realloc), i = w;
    a().setInt32(t + 4 * 1, i, true), a().setInt32(t + 4 * 0, u, true);
  }, arguments);
}
__name(mt, "mt");
function Rt() {
  return c(function(t, e) {
    return Reflect.get(t, e);
  }, arguments);
}
__name(Rt, "Rt");
function Ft(t) {
  let e = t.done;
  return s(e) ? 16777215 : e ? 1 : 0;
}
__name(Ft, "Ft");
function St(t) {
  return t.value;
}
__name(St, "St");
function jt(t, e) {
  return t[e];
}
__name(jt, "jt");
function It(t, e) {
  return t[e];
}
__name(It, "It");
function Et(t) {
  return t.headers;
}
__name(Et, "Et");
function Ot(t) {
  return t.headers;
}
__name(Ot, "Ot");
function kt(t) {
  let e;
  try {
    e = t instanceof ArrayBuffer;
  } catch {
    e = false;
  }
  return e;
}
__name(kt, "kt");
function zt(t) {
  let e;
  try {
    e = t instanceof Error;
  } catch {
    e = false;
  }
  return e;
}
__name(zt, "zt");
function At(t) {
  let e;
  try {
    e = t instanceof ReadableStream;
  } catch {
    e = false;
  }
  return e;
}
__name(At, "At");
function Tt(t) {
  let e;
  try {
    e = t instanceof Response;
  } catch {
    e = false;
  }
  return e;
}
__name(Tt, "Tt");
function qt(t) {
  let e;
  try {
    e = t instanceof Uint8Array;
  } catch {
    e = false;
  }
  return e;
}
__name(qt, "qt");
function Lt() {
  return c(function(t) {
    return t.json();
  }, arguments);
}
__name(Lt, "Lt");
function Mt(t) {
  return t.length;
}
__name(Mt, "Mt");
function Ut(t) {
  console.log(t);
}
__name(Ut, "Ut");
function Dt(t, e) {
  let n = e.method, r = x(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(Dt, "Dt");
function Ct(t) {
  return p.__wrap(t);
}
__name(Ct, "Ct");
function Wt(t) {
  return t.name;
}
__name(Wt, "Wt");
function Bt() {
  return c(function() {
    return new Headers();
  }, arguments);
}
__name(Bt, "Bt");
function Nt(t, e) {
  try {
    var n = { a: t, b: e }, r = /* @__PURE__ */ __name((u, i) => {
      let f = n.a;
      n.a = 0;
      try {
        return $(f, n.b, u, i);
      } finally {
        n.a = f;
      }
    }, "r");
    return new Promise(r);
  } finally {
    n.a = n.b = 0;
  }
}
__name(Nt, "Nt");
function $t() {
  return new Object();
}
__name($t, "$t");
function Pt() {
  return /* @__PURE__ */ new Map();
}
__name(Pt, "Pt");
function Vt() {
  return new Array();
}
__name(Vt, "Vt");
function vt(t) {
  return new Uint8Array(t);
}
__name(vt, "vt");
function Gt(t, e) {
  return new Error(g(t, e));
}
__name(Gt, "Gt");
function Ht(t, e) {
  return new Function(g(t, e));
}
__name(Ht, "Ht");
function Jt(t, e, n) {
  return new Uint8Array(t, e >>> 0, n >>> 0);
}
__name(Jt, "Jt");
function Xt(t) {
  return new Uint8Array(t >>> 0);
}
__name(Xt, "Xt");
function Kt() {
  return c(function(t, e) {
    return new Response(t, e);
  }, arguments);
}
__name(Kt, "Kt");
function Qt() {
  return c(function(t, e) {
    return new Response(t, e);
  }, arguments);
}
__name(Qt, "Qt");
function Yt() {
  return c(function(t, e, n) {
    return new Response(t === 0 ? void 0 : g(t, e), n);
  }, arguments);
}
__name(Yt, "Yt");
function Zt() {
  return c(function(t, e, n) {
    return new Request(g(t, e), n);
  }, arguments);
}
__name(Zt, "Zt");
function te(t) {
  queueMicrotask(t);
}
__name(te, "te");
function ee(t) {
  return t.queueMicrotask;
}
__name(ee, "ee");
function ne(t) {
  return t.read();
}
__name(ne, "ne");
function re(t) {
  t.releaseLock();
}
__name(re, "re");
function _e(t) {
  return Promise.resolve(t);
}
__name(_e, "_e");
function oe() {
  return c(function(t, e) {
    t.respond(e >>> 0);
  }, arguments);
}
__name(oe, "oe");
function ce() {
  return c(function(t, e, n, r, o) {
    t.set(g(e, n), g(r, o));
  }, arguments);
}
__name(ce, "ce");
function ie(t, e, n) {
  t[e >>> 0] = n;
}
__name(ie, "ie");
function se(t, e, n) {
  t[e] = n;
}
__name(se, "se");
function ue(t, e, n) {
  t[e] = n;
}
__name(ue, "ue");
function fe(t, e, n) {
  t.set(e, n >>> 0);
}
__name(fe, "fe");
function ae(t, e, n) {
  return t.set(e, n);
}
__name(ae, "ae");
function ge() {
  return c(function(t, e, n) {
    return Reflect.set(t, e, n);
  }, arguments);
}
__name(ge, "ge");
function be(t, e) {
  t.body = e;
}
__name(be, "be");
function de(t, e) {
  t.headers = e;
}
__name(de, "de");
function we(t, e) {
  t.headers = e;
}
__name(we, "we");
function le(t, e, n) {
  t.method = g(e, n);
}
__name(le, "le");
function pe(t, e) {
  t.redirect = G[e];
}
__name(pe, "pe");
function xe(t, e) {
  t.signal = e;
}
__name(xe, "xe");
function ye(t, e) {
  t.status = e;
}
__name(ye, "ye");
function he() {
  let t = typeof global > "u" ? null : global;
  return s(t) ? 0 : b(t);
}
__name(he, "he");
function me() {
  let t = typeof globalThis > "u" ? null : globalThis;
  return s(t) ? 0 : b(t);
}
__name(me, "me");
function Re() {
  let t = typeof self > "u" ? null : self;
  return s(t) ? 0 : b(t);
}
__name(Re, "Re");
function Fe() {
  let t = typeof window > "u" ? null : window;
  return s(t) ? 0 : b(t);
}
__name(Fe, "Fe");
function Se(t) {
  return t.status;
}
__name(Se, "Se");
function je() {
  return c(function(t) {
    return JSON.stringify(t);
  }, arguments);
}
__name(je, "je");
function Ie(t, e) {
  return t.then(e);
}
__name(Ie, "Ie");
function Ee(t, e, n) {
  return t.then(e, n);
}
__name(Ee, "Ee");
function Oe(t) {
  return t.toString();
}
__name(Oe, "Oe");
function ke(t, e) {
  let n = e.url, r = x(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(ke, "ke");
function ze(t) {
  let e = t.view;
  return s(e) ? 0 : b(e);
}
__name(ze, "ze");
function Ae() {
  return c(function(t) {
    let e = t.webSocket;
    return s(e) ? 0 : b(e);
  }, arguments);
}
__name(Ae, "Ae");
function Te(t) {
  return t;
}
__name(Te, "Te");
function qe(t) {
  return BigInt.asUintN(64, t);
}
__name(qe, "qe");
function Le(t) {
  let e = t;
  return typeof e == "boolean" ? e ? 1 : 0 : 2;
}
__name(Le, "Le");
function Me(t) {
  let e = t.original;
  return e.cnt-- == 1 ? (e.a = 0, true) : false;
}
__name(Me, "Me");
function Ue(t, e, n) {
  return B(t, e, 366, N);
}
__name(Ue, "Ue");
function De(t, e) {
  let n = S(e), r = x(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(De, "De");
function Ce(t, e) {
  return new Error(g(t, e));
}
__name(Ce, "Ce");
function We(t, e) {
  return t in e;
}
__name(We, "We");
function Be() {
  let t = _.__wbindgen_export_3, e = t.grow(4);
  t.set(0, void 0), t.set(e + 0, void 0), t.set(e + 1, null), t.set(e + 2, true), t.set(e + 3, false);
}
__name(Be, "Be");
function Ne(t) {
  return typeof t == "function";
}
__name(Ne, "Ne");
function $e(t) {
  return t === null;
}
__name($e, "$e");
function Pe(t) {
  let e = t;
  return typeof e == "object" && e !== null;
}
__name(Pe, "Pe");
function Ve(t) {
  return typeof t == "string";
}
__name(Ve, "Ve");
function ve(t) {
  return t === void 0;
}
__name(ve, "ve");
function Ge(t, e) {
  return t == e;
}
__name(Ge, "Ge");
function He() {
  return _.memory;
}
__name(He, "He");
function Je(t, e) {
  let n = e, r = typeof n == "number" ? n : void 0;
  a().setFloat64(t + 8 * 1, s(r) ? 0 : r, true), a().setInt32(t + 4 * 0, !s(r), true);
}
__name(Je, "Je");
function Xe(t) {
  return t;
}
__name(Xe, "Xe");
function Ke(t, e) {
  let n = e, r = typeof n == "string" ? n : void 0;
  var o = s(r) ? 0 : x(r, _.__wbindgen_malloc, _.__wbindgen_realloc), u = w;
  a().setInt32(t + 4 * 1, u, true), a().setInt32(t + 4 * 0, o, true);
}
__name(Ke, "Ke");
function Qe(t, e) {
  return g(t, e);
}
__name(Qe, "Qe");
function Ye(t, e) {
  throw new Error(g(t, e));
}
__name(Ye, "Ye");
var L = new WebAssembly.Instance(Ze, { "./index_bg.js": d });
k(L.exports);
L.exports.__wbindgen_start?.();
var F = class extends tn {
  static {
    __name(this, "F");
  }
  async fetch(e) {
    return await z(e, this.env, this.ctx);
  }
  async queue(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
  async scheduled(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
};
var en = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(d).map((t) => {
  en.includes(t) | t.startsWith("__") || (F.prototype[t] = d[t]);
});
var on = F;
export {
  j as IntoUnderlyingByteSource,
  I as IntoUnderlyingSink,
  E as IntoUnderlyingSource,
  p as MinifyConfig,
  P as PolishConfig,
  O as R2Range,
  V as RequestRedirect,
  Q as __wbg_String_8f0eb39a4a4c2f66,
  Y as __wbg_body_0b8fd1fe671660df,
  Z as __wbg_buffer_09165b52af8c5237,
  tt as __wbg_buffer_609cc3eee51ed158,
  et as __wbg_byobRequest_77d9adf63337edfb,
  nt as __wbg_byteLength_e674b853d9c77e1d,
  rt as __wbg_byteOffset_fd862df290ef848d,
  _t as __wbg_call_672a4d21634d4a24,
  ot as __wbg_call_7cccdd69e0791ae2,
  ct as __wbg_call_833bed5770ea2041,
  it as __wbg_call_b8adc8b1d0a0d8eb,
  st as __wbg_cancel_8a308660caa6cadf,
  ut as __wbg_catch_a6e601879b2610e9,
  ft as __wbg_cause_9940c4e8dfcd5129,
  at as __wbg_cf_123509d53a2ea003,
  gt as __wbg_cf_abc51304c8a6868c,
  bt as __wbg_close_304cc1fef3466669,
  dt as __wbg_close_5ce03e29be453811,
  wt as __wbg_constructor_9fd96f589d65d4e5,
  lt as __wbg_enqueue_bb16ba72f537dc9e,
  pt as __wbg_error_524f506f44df1645,
  xt as __wbg_fetch_07cd86dd296a5a63,
  yt as __wbg_fetch_79398949f1862502,
  ht as __wbg_getReader_48e00749fe3f6089,
  mt as __wbg_get_123509460060ab98,
  Rt as __wbg_get_67b2ba62fc30de12,
  Ft as __wbg_getdone_d47073731acd3e74,
  St as __wbg_getvalue_009dcd63692bee1f,
  jt as __wbg_getwithrefkey_1dc361bd10053bfe,
  It as __wbg_getwithrefkey_6550b2c093d2eb18,
  Et as __wbg_headers_7852a8ea641c1379,
  Ot as __wbg_headers_9cb51cfd2ac780a4,
  kt as __wbg_instanceof_ArrayBuffer_e14585432e3737fc,
  zt as __wbg_instanceof_Error_4d54113b22d20306,
  At as __wbg_instanceof_ReadableStream_87eac785b90f3611,
  Tt as __wbg_instanceof_Response_f2cc20d9f7dfd644,
  qt as __wbg_instanceof_Uint8Array_17156bcf118086a9,
  Lt as __wbg_json_a00f187c0be01957,
  Mt as __wbg_length_a446193dc22c12f8,
  Ut as __wbg_log_c222819a41e063d3,
  Dt as __wbg_method_3dcc854b644c5a56,
  Ct as __wbg_minifyconfig_new,
  Wt as __wbg_name_16617c8e9d4188ac,
  Bt as __wbg_new_018dcc2d6c8c2f6a,
  Nt as __wbg_new_23a2665fac83c611,
  $t as __wbg_new_405e22f390576ce2,
  Pt as __wbg_new_5e0be73521bc8c17,
  Vt as __wbg_new_78feb108b6472713,
  vt as __wbg_new_a12002a7f91c75be,
  Gt as __wbg_new_c68d7209be747379,
  Ht as __wbg_newnoargs_105ed471475aaf50,
  Jt as __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a,
  Xt as __wbg_newwithlength_a381634e90c276d4,
  Kt as __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1,
  Qt as __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e,
  Yt as __wbg_newwithoptstrandinit_615a266ef226c260,
  Zt as __wbg_newwithstrandinit_06c535e0a867c635,
  te as __wbg_queueMicrotask_97d92b4fcc8a61c5,
  ee as __wbg_queueMicrotask_d3219def82552485,
  ne as __wbg_read_a2434af1186cb56c,
  re as __wbg_releaseLock_091899af97991d2e,
  _e as __wbg_resolve_4851785c9c5f573d,
  oe as __wbg_respond_1f279fa9f8edcb1c,
  ce as __wbg_set_11cd83f45504cedf,
  ie as __wbg_set_37837023f3d740e8,
  se as __wbg_set_3807d5f0bfc24aa7,
  ue as __wbg_set_3f1d0b984ed272ed,
  fe as __wbg_set_65595bdd868b3009,
  ae as __wbg_set_8fc6bf8a5b1071d1,
  ge as __wbg_set_bb8cecf6a62b9f46,
  k as __wbg_set_wasm,
  be as __wbg_setbody_5923b78a95eedf29,
  de as __wbg_setheaders_3b47c898e8de6d44,
  we as __wbg_setheaders_834c0bdb6a8949ad,
  le as __wbg_setmethod_3c5280fe5d890842,
  pe as __wbg_setredirect_40e6a7f717a2f86a,
  xe as __wbg_setsignal_75b21ef3a81de905,
  ye as __wbg_setstatus_51b4fc011091cbb3,
  he as __wbg_static_accessor_GLOBAL_88a902d13a557d07,
  me as __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0,
  Re as __wbg_static_accessor_SELF_37c5d418e4bf5819,
  Fe as __wbg_static_accessor_WINDOW_5de37043a91a9c40,
  Se as __wbg_status_f6360336ca686bf0,
  je as __wbg_stringify_f7ed6987935b4a24,
  Ie as __wbg_then_44b73946d2fb3e7d,
  Ee as __wbg_then_48b406749878a531,
  Oe as __wbg_toString_c813bbd34d063839,
  ke as __wbg_url_8f9653b899456042,
  ze as __wbg_view_fd8a56e8983f448d,
  Ae as __wbg_webSocket_4a16d2250705e19d,
  Te as __wbindgen_bigint_from_i64,
  qe as __wbindgen_bigint_from_u64,
  Le as __wbindgen_boolean_get,
  Me as __wbindgen_cb_drop,
  Ue as __wbindgen_closure_wrapper1830,
  De as __wbindgen_debug_string,
  Ce as __wbindgen_error_new,
  We as __wbindgen_in,
  Be as __wbindgen_init_externref_table,
  Ne as __wbindgen_is_function,
  $e as __wbindgen_is_null,
  Pe as __wbindgen_is_object,
  Ve as __wbindgen_is_string,
  ve as __wbindgen_is_undefined,
  Ge as __wbindgen_jsval_loose_eq,
  He as __wbindgen_memory,
  Je as __wbindgen_number_get,
  Xe as __wbindgen_number_new,
  Ke as __wbindgen_string_get,
  Qe as __wbindgen_string_new,
  Ye as __wbindgen_throw,
  on as default,
  z as fetch,
  Ze as wasmModule
};
//# sourceMappingURL=shim.js.map
