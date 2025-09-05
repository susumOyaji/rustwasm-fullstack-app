var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/worker/shim.mjs
import Ve from "./47c85508659f7f318f6dd5c40ce6c9e03f28b4b1-index.wasm";
import { WorkerEntrypoint as Ge } from "cloudflare:workers";
var M = Object.defineProperty;
var U = /* @__PURE__ */ __name((t, e) => {
  for (var n in e) M(t, n, { get: e[n], enumerable: true });
}, "U");
var d = {};
U(d, { IntoUnderlyingByteSource: /* @__PURE__ */ __name(() => j, "IntoUnderlyingByteSource"), IntoUnderlyingSink: /* @__PURE__ */ __name(() => E, "IntoUnderlyingSink"), IntoUnderlyingSource: /* @__PURE__ */ __name(() => k, "IntoUnderlyingSource"), MinifyConfig: /* @__PURE__ */ __name(() => p, "MinifyConfig"), PolishConfig: /* @__PURE__ */ __name(() => P, "PolishConfig"), R2Range: /* @__PURE__ */ __name(() => z, "R2Range"), RequestRedirect: /* @__PURE__ */ __name(() => V, "RequestRedirect"), __wbg_String_8f0eb39a4a4c2f66: /* @__PURE__ */ __name(() => Q, "__wbg_String_8f0eb39a4a4c2f66"), __wbg_body_0b8fd1fe671660df: /* @__PURE__ */ __name(() => Y, "__wbg_body_0b8fd1fe671660df"), __wbg_buffer_09165b52af8c5237: /* @__PURE__ */ __name(() => Z, "__wbg_buffer_09165b52af8c5237"), __wbg_buffer_609cc3eee51ed158: /* @__PURE__ */ __name(() => tt, "__wbg_buffer_609cc3eee51ed158"), __wbg_byobRequest_77d9adf63337edfb: /* @__PURE__ */ __name(() => et, "__wbg_byobRequest_77d9adf63337edfb"), __wbg_byteLength_e674b853d9c77e1d: /* @__PURE__ */ __name(() => nt, "__wbg_byteLength_e674b853d9c77e1d"), __wbg_byteOffset_fd862df290ef848d: /* @__PURE__ */ __name(() => rt, "__wbg_byteOffset_fd862df290ef848d"), __wbg_call_672a4d21634d4a24: /* @__PURE__ */ __name(() => _t, "__wbg_call_672a4d21634d4a24"), __wbg_call_7cccdd69e0791ae2: /* @__PURE__ */ __name(() => ot, "__wbg_call_7cccdd69e0791ae2"), __wbg_call_833bed5770ea2041: /* @__PURE__ */ __name(() => it, "__wbg_call_833bed5770ea2041"), __wbg_cancel_8a308660caa6cadf: /* @__PURE__ */ __name(() => ct, "__wbg_cancel_8a308660caa6cadf"), __wbg_catch_a6e601879b2610e9: /* @__PURE__ */ __name(() => st, "__wbg_catch_a6e601879b2610e9"), __wbg_cause_9940c4e8dfcd5129: /* @__PURE__ */ __name(() => ut, "__wbg_cause_9940c4e8dfcd5129"), __wbg_cf_123509d53a2ea003: /* @__PURE__ */ __name(() => ft, "__wbg_cf_123509d53a2ea003"), __wbg_cf_abc51304c8a6868c: /* @__PURE__ */ __name(() => at, "__wbg_cf_abc51304c8a6868c"), __wbg_close_304cc1fef3466669: /* @__PURE__ */ __name(() => gt, "__wbg_close_304cc1fef3466669"), __wbg_close_5ce03e29be453811: /* @__PURE__ */ __name(() => bt, "__wbg_close_5ce03e29be453811"), __wbg_enqueue_bb16ba72f537dc9e: /* @__PURE__ */ __name(() => dt, "__wbg_enqueue_bb16ba72f537dc9e"), __wbg_error_524f506f44df1645: /* @__PURE__ */ __name(() => wt, "__wbg_error_524f506f44df1645"), __wbg_fetch_07cd86dd296a5a63: /* @__PURE__ */ __name(() => lt, "__wbg_fetch_07cd86dd296a5a63"), __wbg_fetch_79398949f1862502: /* @__PURE__ */ __name(() => pt, "__wbg_fetch_79398949f1862502"), __wbg_getReader_48e00749fe3f6089: /* @__PURE__ */ __name(() => yt, "__wbg_getReader_48e00749fe3f6089"), __wbg_get_67b2ba62fc30de12: /* @__PURE__ */ __name(() => xt, "__wbg_get_67b2ba62fc30de12"), __wbg_getdone_d47073731acd3e74: /* @__PURE__ */ __name(() => ht, "__wbg_getdone_d47073731acd3e74"), __wbg_getvalue_009dcd63692bee1f: /* @__PURE__ */ __name(() => mt, "__wbg_getvalue_009dcd63692bee1f"), __wbg_getwithrefkey_1dc361bd10053bfe: /* @__PURE__ */ __name(() => Rt, "__wbg_getwithrefkey_1dc361bd10053bfe"), __wbg_getwithrefkey_6550b2c093d2eb18: /* @__PURE__ */ __name(() => Ft, "__wbg_getwithrefkey_6550b2c093d2eb18"), __wbg_headers_7852a8ea641c1379: /* @__PURE__ */ __name(() => St, "__wbg_headers_7852a8ea641c1379"), __wbg_headers_9cb51cfd2ac780a4: /* @__PURE__ */ __name(() => jt, "__wbg_headers_9cb51cfd2ac780a4"), __wbg_instanceof_ArrayBuffer_e14585432e3737fc: /* @__PURE__ */ __name(() => Et, "__wbg_instanceof_ArrayBuffer_e14585432e3737fc"), __wbg_instanceof_Error_4d54113b22d20306: /* @__PURE__ */ __name(() => kt, "__wbg_instanceof_Error_4d54113b22d20306"), __wbg_instanceof_ReadableStream_87eac785b90f3611: /* @__PURE__ */ __name(() => zt, "__wbg_instanceof_ReadableStream_87eac785b90f3611"), __wbg_instanceof_Response_f2cc20d9f7dfd644: /* @__PURE__ */ __name(() => It, "__wbg_instanceof_Response_f2cc20d9f7dfd644"), __wbg_instanceof_Uint8Array_17156bcf118086a9: /* @__PURE__ */ __name(() => Ot, "__wbg_instanceof_Uint8Array_17156bcf118086a9"), __wbg_json_a00f187c0be01957: /* @__PURE__ */ __name(() => Tt, "__wbg_json_a00f187c0be01957"), __wbg_length_a446193dc22c12f8: /* @__PURE__ */ __name(() => At, "__wbg_length_a446193dc22c12f8"), __wbg_method_3dcc854b644c5a56: /* @__PURE__ */ __name(() => qt, "__wbg_method_3dcc854b644c5a56"), __wbg_minifyconfig_new: /* @__PURE__ */ __name(() => Lt, "__wbg_minifyconfig_new"), __wbg_new_018dcc2d6c8c2f6a: /* @__PURE__ */ __name(() => Mt, "__wbg_new_018dcc2d6c8c2f6a"), __wbg_new_23a2665fac83c611: /* @__PURE__ */ __name(() => Ut, "__wbg_new_23a2665fac83c611"), __wbg_new_405e22f390576ce2: /* @__PURE__ */ __name(() => Dt, "__wbg_new_405e22f390576ce2"), __wbg_new_5e0be73521bc8c17: /* @__PURE__ */ __name(() => Ct, "__wbg_new_5e0be73521bc8c17"), __wbg_new_a12002a7f91c75be: /* @__PURE__ */ __name(() => Wt, "__wbg_new_a12002a7f91c75be"), __wbg_new_c68d7209be747379: /* @__PURE__ */ __name(() => Bt, "__wbg_new_c68d7209be747379"), __wbg_newnoargs_105ed471475aaf50: /* @__PURE__ */ __name(() => $t, "__wbg_newnoargs_105ed471475aaf50"), __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a: /* @__PURE__ */ __name(() => Nt, "__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a"), __wbg_newwithlength_a381634e90c276d4: /* @__PURE__ */ __name(() => Pt, "__wbg_newwithlength_a381634e90c276d4"), __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1: /* @__PURE__ */ __name(() => Vt, "__wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1"), __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e: /* @__PURE__ */ __name(() => Gt, "__wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e"), __wbg_newwithoptstrandinit_615a266ef226c260: /* @__PURE__ */ __name(() => Ht, "__wbg_newwithoptstrandinit_615a266ef226c260"), __wbg_newwithstrandinit_06c535e0a867c635: /* @__PURE__ */ __name(() => Xt, "__wbg_newwithstrandinit_06c535e0a867c635"), __wbg_queueMicrotask_97d92b4fcc8a61c5: /* @__PURE__ */ __name(() => vt, "__wbg_queueMicrotask_97d92b4fcc8a61c5"), __wbg_queueMicrotask_d3219def82552485: /* @__PURE__ */ __name(() => Jt, "__wbg_queueMicrotask_d3219def82552485"), __wbg_read_a2434af1186cb56c: /* @__PURE__ */ __name(() => Kt, "__wbg_read_a2434af1186cb56c"), __wbg_releaseLock_091899af97991d2e: /* @__PURE__ */ __name(() => Qt, "__wbg_releaseLock_091899af97991d2e"), __wbg_resolve_4851785c9c5f573d: /* @__PURE__ */ __name(() => Yt, "__wbg_resolve_4851785c9c5f573d"), __wbg_respond_1f279fa9f8edcb1c: /* @__PURE__ */ __name(() => Zt, "__wbg_respond_1f279fa9f8edcb1c"), __wbg_set_11cd83f45504cedf: /* @__PURE__ */ __name(() => te, "__wbg_set_11cd83f45504cedf"), __wbg_set_3807d5f0bfc24aa7: /* @__PURE__ */ __name(() => ee, "__wbg_set_3807d5f0bfc24aa7"), __wbg_set_3f1d0b984ed272ed: /* @__PURE__ */ __name(() => ne, "__wbg_set_3f1d0b984ed272ed"), __wbg_set_65595bdd868b3009: /* @__PURE__ */ __name(() => re, "__wbg_set_65595bdd868b3009"), __wbg_set_8fc6bf8a5b1071d1: /* @__PURE__ */ __name(() => _e, "__wbg_set_8fc6bf8a5b1071d1"), __wbg_set_bb8cecf6a62b9f46: /* @__PURE__ */ __name(() => oe, "__wbg_set_bb8cecf6a62b9f46"), __wbg_set_wasm: /* @__PURE__ */ __name(() => I, "__wbg_set_wasm"), __wbg_setbody_5923b78a95eedf29: /* @__PURE__ */ __name(() => ie, "__wbg_setbody_5923b78a95eedf29"), __wbg_setheaders_3b47c898e8de6d44: /* @__PURE__ */ __name(() => ce, "__wbg_setheaders_3b47c898e8de6d44"), __wbg_setheaders_834c0bdb6a8949ad: /* @__PURE__ */ __name(() => se, "__wbg_setheaders_834c0bdb6a8949ad"), __wbg_setmethod_3c5280fe5d890842: /* @__PURE__ */ __name(() => ue, "__wbg_setmethod_3c5280fe5d890842"), __wbg_setredirect_40e6a7f717a2f86a: /* @__PURE__ */ __name(() => fe, "__wbg_setredirect_40e6a7f717a2f86a"), __wbg_setsignal_75b21ef3a81de905: /* @__PURE__ */ __name(() => ae, "__wbg_setsignal_75b21ef3a81de905"), __wbg_setstatus_51b4fc011091cbb3: /* @__PURE__ */ __name(() => ge, "__wbg_setstatus_51b4fc011091cbb3"), __wbg_static_accessor_GLOBAL_88a902d13a557d07: /* @__PURE__ */ __name(() => be, "__wbg_static_accessor_GLOBAL_88a902d13a557d07"), __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0: /* @__PURE__ */ __name(() => de, "__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0"), __wbg_static_accessor_SELF_37c5d418e4bf5819: /* @__PURE__ */ __name(() => we, "__wbg_static_accessor_SELF_37c5d418e4bf5819"), __wbg_static_accessor_WINDOW_5de37043a91a9c40: /* @__PURE__ */ __name(() => le, "__wbg_static_accessor_WINDOW_5de37043a91a9c40"), __wbg_status_f6360336ca686bf0: /* @__PURE__ */ __name(() => pe, "__wbg_status_f6360336ca686bf0"), __wbg_then_44b73946d2fb3e7d: /* @__PURE__ */ __name(() => ye, "__wbg_then_44b73946d2fb3e7d"), __wbg_then_48b406749878a531: /* @__PURE__ */ __name(() => xe, "__wbg_then_48b406749878a531"), __wbg_toString_c813bbd34d063839: /* @__PURE__ */ __name(() => he, "__wbg_toString_c813bbd34d063839"), __wbg_url_8f9653b899456042: /* @__PURE__ */ __name(() => me, "__wbg_url_8f9653b899456042"), __wbg_view_fd8a56e8983f448d: /* @__PURE__ */ __name(() => Re, "__wbg_view_fd8a56e8983f448d"), __wbg_webSocket_4a16d2250705e19d: /* @__PURE__ */ __name(() => Fe, "__wbg_webSocket_4a16d2250705e19d"), __wbindgen_bigint_from_u64: /* @__PURE__ */ __name(() => Se, "__wbindgen_bigint_from_u64"), __wbindgen_boolean_get: /* @__PURE__ */ __name(() => je, "__wbindgen_boolean_get"), __wbindgen_cb_drop: /* @__PURE__ */ __name(() => Ee, "__wbindgen_cb_drop"), __wbindgen_closure_wrapper1084: /* @__PURE__ */ __name(() => ke, "__wbindgen_closure_wrapper1084"), __wbindgen_debug_string: /* @__PURE__ */ __name(() => ze, "__wbindgen_debug_string"), __wbindgen_error_new: /* @__PURE__ */ __name(() => Ie, "__wbindgen_error_new"), __wbindgen_in: /* @__PURE__ */ __name(() => Oe, "__wbindgen_in"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(() => Te, "__wbindgen_init_externref_table"), __wbindgen_is_function: /* @__PURE__ */ __name(() => Ae, "__wbindgen_is_function"), __wbindgen_is_null: /* @__PURE__ */ __name(() => qe, "__wbindgen_is_null"), __wbindgen_is_object: /* @__PURE__ */ __name(() => Le, "__wbindgen_is_object"), __wbindgen_is_string: /* @__PURE__ */ __name(() => Me, "__wbindgen_is_string"), __wbindgen_is_undefined: /* @__PURE__ */ __name(() => Ue, "__wbindgen_is_undefined"), __wbindgen_jsval_loose_eq: /* @__PURE__ */ __name(() => De, "__wbindgen_jsval_loose_eq"), __wbindgen_memory: /* @__PURE__ */ __name(() => Ce, "__wbindgen_memory"), __wbindgen_number_get: /* @__PURE__ */ __name(() => We, "__wbindgen_number_get"), __wbindgen_number_new: /* @__PURE__ */ __name(() => Be, "__wbindgen_number_new"), __wbindgen_string_get: /* @__PURE__ */ __name(() => $e, "__wbindgen_string_get"), __wbindgen_string_new: /* @__PURE__ */ __name(() => Ne, "__wbindgen_string_new"), __wbindgen_throw: /* @__PURE__ */ __name(() => Pe, "__wbindgen_throw"), fetch: /* @__PURE__ */ __name(() => O, "fetch") });
var _;
function I(t) {
  _ = t;
}
__name(I, "I");
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
function y(t, e, n) {
  if (n === void 0) {
    let u = R.encode(t), x = e(u.length, 1) >>> 0;
    return m().subarray(x, x + u.length).set(u), w = u.length, x;
  }
  let r = t.length, o = e(r, 1) >>> 0, f = m(), i = 0;
  for (; i < r; i++) {
    let u = t.charCodeAt(i);
    if (u > 127) break;
    f[o + i] = u;
  }
  if (i !== r) {
    i !== 0 && (t = t.slice(i)), o = n(o, r, r = i + t.length * 3, 1) >>> 0;
    let u = m().subarray(o + i, o + r);
    i += C(t, u).written, o = n(o, r, i, 1) >>> 0;
  }
  return w = i, o;
}
__name(y, "y");
var l = null;
function a() {
  return (l === null || l.buffer.detached === true || l.buffer.detached === void 0 && l.buffer !== _.memory.buffer) && (l = new DataView(_.memory.buffer)), l;
}
__name(a, "a");
function c(t) {
  return t == null;
}
__name(c, "c");
function b(t) {
  let e = _.__externref_table_alloc();
  return _.__wbindgen_export_3.set(e, t), e;
}
__name(b, "b");
function s(t, e) {
  try {
    return t.apply(this, e);
  } catch (n) {
    let r = b(n);
    _.__wbindgen_exn_store(r);
  }
}
__name(s, "s");
var W = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var q = new W("utf-8", { ignoreBOM: true, fatal: true });
q.decode();
function g(t, e) {
  return t = t >>> 0, q.decode(m().subarray(t, t + e));
}
__name(g, "g");
var T = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => {
  _.__wbindgen_export_5.get(t.dtor)(t.a, t.b);
});
function B(t, e, n, r) {
  let o = { a: t, b: e, cnt: 1, dtor: n }, f = /* @__PURE__ */ __name((...i) => {
    o.cnt++;
    let u = o.a;
    o.a = 0;
    try {
      return r(u, o.b, ...i);
    } finally {
      --o.cnt === 0 ? (_.__wbindgen_export_5.get(o.dtor)(u, o.b), T.unregister(o)) : o.a = u;
    }
  }, "f");
  return f.original = o, T.register(f, o, o), f;
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
    let o = t.length, f = "[";
    o > 0 && (f += S(t[0]));
    for (let i = 1; i < o; i++) f += ", " + S(t[i]);
    return f += "]", f;
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
function O(t, e, n) {
  return _.fetch(t, e, n);
}
__name(O, "O");
function $(t, e, n) {
  _.closure166_externref_shim(t, e, n);
}
__name($, "$");
function N(t, e, n, r) {
  _.closure209_externref_shim(t, e, n, r);
}
__name(N, "N");
var P = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var V = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var G = ["bytes"];
var H = ["follow", "error", "manual"];
var X = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_intounderlyingbytesource_free(t >>> 0, 1));
var j = class {
  static {
    __name(this, "j");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, X.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    _.__wbg_intounderlyingbytesource_free(e, 0);
  }
  get type() {
    let e = _.intounderlyingbytesource_type(this.__wbg_ptr);
    return G[e];
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
var v = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_intounderlyingsink_free(t >>> 0, 1));
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, v.unregister(this), e;
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
var J = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_intounderlyingsource_free(t >>> 0, 1));
var k = class {
  static {
    __name(this, "k");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, J.unregister(this), e;
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
var A = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((t) => _.__wbg_minifyconfig_free(t >>> 0, 1));
var p = class {
  static {
    __name(this, "p");
  }
  static __wrap(e) {
    e = e >>> 0;
    let n = Object.create(p.prototype);
    return n.__wbg_ptr = e, A.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, A.unregister(this), e;
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
var z = class {
  static {
    __name(this, "z");
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
    _.__wbg_set_r2range_offset(this.__wbg_ptr, !c(e), c(e) ? 0 : e);
  }
  get length() {
    let e = _.__wbg_get_r2range_length(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set length(e) {
    _.__wbg_set_r2range_length(this.__wbg_ptr, !c(e), c(e) ? 0 : e);
  }
  get suffix() {
    let e = _.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return e[0] === 0 ? void 0 : e[1];
  }
  set suffix(e) {
    _.__wbg_set_r2range_suffix(this.__wbg_ptr, !c(e), c(e) ? 0 : e);
  }
};
function Q(t, e) {
  let n = String(e), r = y(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(Q, "Q");
function Y(t) {
  let e = t.body;
  return c(e) ? 0 : b(e);
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
  return c(e) ? 0 : b(e);
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
  return s(function(t, e) {
    return t.call(e);
  }, arguments);
}
__name(_t, "_t");
function ot() {
  return s(function(t, e, n) {
    return t.call(e, n);
  }, arguments);
}
__name(ot, "ot");
function it() {
  return s(function(t, e, n, r) {
    return t.call(e, n, r);
  }, arguments);
}
__name(it, "it");
function ct(t) {
  return t.cancel();
}
__name(ct, "ct");
function st(t, e) {
  return t.catch(e);
}
__name(st, "st");
function ut(t) {
  return t.cause;
}
__name(ut, "ut");
function ft() {
  return s(function(t) {
    let e = t.cf;
    return c(e) ? 0 : b(e);
  }, arguments);
}
__name(ft, "ft");
function at() {
  return s(function(t) {
    let e = t.cf;
    return c(e) ? 0 : b(e);
  }, arguments);
}
__name(at, "at");
function gt() {
  return s(function(t) {
    t.close();
  }, arguments);
}
__name(gt, "gt");
function bt() {
  return s(function(t) {
    t.close();
  }, arguments);
}
__name(bt, "bt");
function dt() {
  return s(function(t, e) {
    t.enqueue(e);
  }, arguments);
}
__name(dt, "dt");
function wt(t) {
  console.error(t);
}
__name(wt, "wt");
function lt(t, e, n) {
  return t.fetch(e, n);
}
__name(lt, "lt");
function pt(t, e, n, r) {
  return t.fetch(g(e, n), r);
}
__name(pt, "pt");
function yt() {
  return s(function(t) {
    return t.getReader();
  }, arguments);
}
__name(yt, "yt");
function xt() {
  return s(function(t, e) {
    return Reflect.get(t, e);
  }, arguments);
}
__name(xt, "xt");
function ht(t) {
  let e = t.done;
  return c(e) ? 16777215 : e ? 1 : 0;
}
__name(ht, "ht");
function mt(t) {
  return t.value;
}
__name(mt, "mt");
function Rt(t, e) {
  return t[e];
}
__name(Rt, "Rt");
function Ft(t, e) {
  return t[e];
}
__name(Ft, "Ft");
function St(t) {
  return t.headers;
}
__name(St, "St");
function jt(t) {
  return t.headers;
}
__name(jt, "jt");
function Et(t) {
  let e;
  try {
    e = t instanceof ArrayBuffer;
  } catch {
    e = false;
  }
  return e;
}
__name(Et, "Et");
function kt(t) {
  let e;
  try {
    e = t instanceof Error;
  } catch {
    e = false;
  }
  return e;
}
__name(kt, "kt");
function zt(t) {
  let e;
  try {
    e = t instanceof ReadableStream;
  } catch {
    e = false;
  }
  return e;
}
__name(zt, "zt");
function It(t) {
  let e;
  try {
    e = t instanceof Response;
  } catch {
    e = false;
  }
  return e;
}
__name(It, "It");
function Ot(t) {
  let e;
  try {
    e = t instanceof Uint8Array;
  } catch {
    e = false;
  }
  return e;
}
__name(Ot, "Ot");
function Tt() {
  return s(function(t) {
    return t.json();
  }, arguments);
}
__name(Tt, "Tt");
function At(t) {
  return t.length;
}
__name(At, "At");
function qt(t, e) {
  let n = e.method, r = y(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(qt, "qt");
function Lt(t) {
  return p.__wrap(t);
}
__name(Lt, "Lt");
function Mt() {
  return s(function() {
    return new Headers();
  }, arguments);
}
__name(Mt, "Mt");
function Ut(t, e) {
  try {
    var n = { a: t, b: e }, r = /* @__PURE__ */ __name((f, i) => {
      let u = n.a;
      n.a = 0;
      try {
        return N(u, n.b, f, i);
      } finally {
        n.a = u;
      }
    }, "r");
    return new Promise(r);
  } finally {
    n.a = n.b = 0;
  }
}
__name(Ut, "Ut");
function Dt() {
  return new Object();
}
__name(Dt, "Dt");
function Ct() {
  return /* @__PURE__ */ new Map();
}
__name(Ct, "Ct");
function Wt(t) {
  return new Uint8Array(t);
}
__name(Wt, "Wt");
function Bt(t, e) {
  return new Error(g(t, e));
}
__name(Bt, "Bt");
function $t(t, e) {
  return new Function(g(t, e));
}
__name($t, "$t");
function Nt(t, e, n) {
  return new Uint8Array(t, e >>> 0, n >>> 0);
}
__name(Nt, "Nt");
function Pt(t) {
  return new Uint8Array(t >>> 0);
}
__name(Pt, "Pt");
function Vt() {
  return s(function(t, e) {
    return new Response(t, e);
  }, arguments);
}
__name(Vt, "Vt");
function Gt() {
  return s(function(t, e) {
    return new Response(t, e);
  }, arguments);
}
__name(Gt, "Gt");
function Ht() {
  return s(function(t, e, n) {
    return new Response(t === 0 ? void 0 : g(t, e), n);
  }, arguments);
}
__name(Ht, "Ht");
function Xt() {
  return s(function(t, e, n) {
    return new Request(g(t, e), n);
  }, arguments);
}
__name(Xt, "Xt");
function vt(t) {
  queueMicrotask(t);
}
__name(vt, "vt");
function Jt(t) {
  return t.queueMicrotask;
}
__name(Jt, "Jt");
function Kt(t) {
  return t.read();
}
__name(Kt, "Kt");
function Qt(t) {
  t.releaseLock();
}
__name(Qt, "Qt");
function Yt(t) {
  return Promise.resolve(t);
}
__name(Yt, "Yt");
function Zt() {
  return s(function(t, e) {
    t.respond(e >>> 0);
  }, arguments);
}
__name(Zt, "Zt");
function te() {
  return s(function(t, e, n, r, o) {
    t.set(g(e, n), g(r, o));
  }, arguments);
}
__name(te, "te");
function ee(t, e, n) {
  t[e] = n;
}
__name(ee, "ee");
function ne(t, e, n) {
  t[e] = n;
}
__name(ne, "ne");
function re(t, e, n) {
  t.set(e, n >>> 0);
}
__name(re, "re");
function _e(t, e, n) {
  return t.set(e, n);
}
__name(_e, "_e");
function oe() {
  return s(function(t, e, n) {
    return Reflect.set(t, e, n);
  }, arguments);
}
__name(oe, "oe");
function ie(t, e) {
  t.body = e;
}
__name(ie, "ie");
function ce(t, e) {
  t.headers = e;
}
__name(ce, "ce");
function se(t, e) {
  t.headers = e;
}
__name(se, "se");
function ue(t, e, n) {
  t.method = g(e, n);
}
__name(ue, "ue");
function fe(t, e) {
  t.redirect = H[e];
}
__name(fe, "fe");
function ae(t, e) {
  t.signal = e;
}
__name(ae, "ae");
function ge(t, e) {
  t.status = e;
}
__name(ge, "ge");
function be() {
  let t = typeof global > "u" ? null : global;
  return c(t) ? 0 : b(t);
}
__name(be, "be");
function de() {
  let t = typeof globalThis > "u" ? null : globalThis;
  return c(t) ? 0 : b(t);
}
__name(de, "de");
function we() {
  let t = typeof self > "u" ? null : self;
  return c(t) ? 0 : b(t);
}
__name(we, "we");
function le() {
  let t = typeof window > "u" ? null : window;
  return c(t) ? 0 : b(t);
}
__name(le, "le");
function pe(t) {
  return t.status;
}
__name(pe, "pe");
function ye(t, e) {
  return t.then(e);
}
__name(ye, "ye");
function xe(t, e, n) {
  return t.then(e, n);
}
__name(xe, "xe");
function he(t) {
  return t.toString();
}
__name(he, "he");
function me(t, e) {
  let n = e.url, r = y(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(me, "me");
function Re(t) {
  let e = t.view;
  return c(e) ? 0 : b(e);
}
__name(Re, "Re");
function Fe() {
  return s(function(t) {
    let e = t.webSocket;
    return c(e) ? 0 : b(e);
  }, arguments);
}
__name(Fe, "Fe");
function Se(t) {
  return BigInt.asUintN(64, t);
}
__name(Se, "Se");
function je(t) {
  let e = t;
  return typeof e == "boolean" ? e ? 1 : 0 : 2;
}
__name(je, "je");
function Ee(t) {
  let e = t.original;
  return e.cnt-- == 1 ? (e.a = 0, true) : false;
}
__name(Ee, "Ee");
function ke(t, e, n) {
  return B(t, e, 167, $);
}
__name(ke, "ke");
function ze(t, e) {
  let n = S(e), r = y(n, _.__wbindgen_malloc, _.__wbindgen_realloc), o = w;
  a().setInt32(t + 4 * 1, o, true), a().setInt32(t + 4 * 0, r, true);
}
__name(ze, "ze");
function Ie(t, e) {
  return new Error(g(t, e));
}
__name(Ie, "Ie");
function Oe(t, e) {
  return t in e;
}
__name(Oe, "Oe");
function Te() {
  let t = _.__wbindgen_export_3, e = t.grow(4);
  t.set(0, void 0), t.set(e + 0, void 0), t.set(e + 1, null), t.set(e + 2, true), t.set(e + 3, false);
}
__name(Te, "Te");
function Ae(t) {
  return typeof t == "function";
}
__name(Ae, "Ae");
function qe(t) {
  return t === null;
}
__name(qe, "qe");
function Le(t) {
  let e = t;
  return typeof e == "object" && e !== null;
}
__name(Le, "Le");
function Me(t) {
  return typeof t == "string";
}
__name(Me, "Me");
function Ue(t) {
  return t === void 0;
}
__name(Ue, "Ue");
function De(t, e) {
  return t == e;
}
__name(De, "De");
function Ce() {
  return _.memory;
}
__name(Ce, "Ce");
function We(t, e) {
  let n = e, r = typeof n == "number" ? n : void 0;
  a().setFloat64(t + 8 * 1, c(r) ? 0 : r, true), a().setInt32(t + 4 * 0, !c(r), true);
}
__name(We, "We");
function Be(t) {
  return t;
}
__name(Be, "Be");
function $e(t, e) {
  let n = e, r = typeof n == "string" ? n : void 0;
  var o = c(r) ? 0 : y(r, _.__wbindgen_malloc, _.__wbindgen_realloc), f = w;
  a().setInt32(t + 4 * 1, f, true), a().setInt32(t + 4 * 0, o, true);
}
__name($e, "$e");
function Ne(t, e) {
  return g(t, e);
}
__name(Ne, "Ne");
function Pe(t, e) {
  throw new Error(g(t, e));
}
__name(Pe, "Pe");
var L = new WebAssembly.Instance(Ve, { "./index_bg.js": d });
I(L.exports);
L.exports.__wbindgen_start?.();
var F = class extends Ge {
  static {
    __name(this, "F");
  }
  async fetch(e) {
    return await O(e, this.env, this.ctx);
  }
  async queue(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
  async scheduled(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
};
var He = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(d).map((t) => {
  He.includes(t) | t.startsWith("__") || (F.prototype[t] = d[t]);
});
var Ke = F;
export {
  j as IntoUnderlyingByteSource,
  E as IntoUnderlyingSink,
  k as IntoUnderlyingSource,
  p as MinifyConfig,
  P as PolishConfig,
  z as R2Range,
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
  it as __wbg_call_833bed5770ea2041,
  ct as __wbg_cancel_8a308660caa6cadf,
  st as __wbg_catch_a6e601879b2610e9,
  ut as __wbg_cause_9940c4e8dfcd5129,
  ft as __wbg_cf_123509d53a2ea003,
  at as __wbg_cf_abc51304c8a6868c,
  gt as __wbg_close_304cc1fef3466669,
  bt as __wbg_close_5ce03e29be453811,
  dt as __wbg_enqueue_bb16ba72f537dc9e,
  wt as __wbg_error_524f506f44df1645,
  lt as __wbg_fetch_07cd86dd296a5a63,
  pt as __wbg_fetch_79398949f1862502,
  yt as __wbg_getReader_48e00749fe3f6089,
  xt as __wbg_get_67b2ba62fc30de12,
  ht as __wbg_getdone_d47073731acd3e74,
  mt as __wbg_getvalue_009dcd63692bee1f,
  Rt as __wbg_getwithrefkey_1dc361bd10053bfe,
  Ft as __wbg_getwithrefkey_6550b2c093d2eb18,
  St as __wbg_headers_7852a8ea641c1379,
  jt as __wbg_headers_9cb51cfd2ac780a4,
  Et as __wbg_instanceof_ArrayBuffer_e14585432e3737fc,
  kt as __wbg_instanceof_Error_4d54113b22d20306,
  zt as __wbg_instanceof_ReadableStream_87eac785b90f3611,
  It as __wbg_instanceof_Response_f2cc20d9f7dfd644,
  Ot as __wbg_instanceof_Uint8Array_17156bcf118086a9,
  Tt as __wbg_json_a00f187c0be01957,
  At as __wbg_length_a446193dc22c12f8,
  qt as __wbg_method_3dcc854b644c5a56,
  Lt as __wbg_minifyconfig_new,
  Mt as __wbg_new_018dcc2d6c8c2f6a,
  Ut as __wbg_new_23a2665fac83c611,
  Dt as __wbg_new_405e22f390576ce2,
  Ct as __wbg_new_5e0be73521bc8c17,
  Wt as __wbg_new_a12002a7f91c75be,
  Bt as __wbg_new_c68d7209be747379,
  $t as __wbg_newnoargs_105ed471475aaf50,
  Nt as __wbg_newwithbyteoffsetandlength_d97e637ebe145a9a,
  Pt as __wbg_newwithlength_a381634e90c276d4,
  Vt as __wbg_newwithoptbuffersourceandinit_fb8ed95e326eb3a1,
  Gt as __wbg_newwithoptreadablestreamandinit_e7fabd7063fd0b3e,
  Ht as __wbg_newwithoptstrandinit_615a266ef226c260,
  Xt as __wbg_newwithstrandinit_06c535e0a867c635,
  vt as __wbg_queueMicrotask_97d92b4fcc8a61c5,
  Jt as __wbg_queueMicrotask_d3219def82552485,
  Kt as __wbg_read_a2434af1186cb56c,
  Qt as __wbg_releaseLock_091899af97991d2e,
  Yt as __wbg_resolve_4851785c9c5f573d,
  Zt as __wbg_respond_1f279fa9f8edcb1c,
  te as __wbg_set_11cd83f45504cedf,
  ee as __wbg_set_3807d5f0bfc24aa7,
  ne as __wbg_set_3f1d0b984ed272ed,
  re as __wbg_set_65595bdd868b3009,
  _e as __wbg_set_8fc6bf8a5b1071d1,
  oe as __wbg_set_bb8cecf6a62b9f46,
  I as __wbg_set_wasm,
  ie as __wbg_setbody_5923b78a95eedf29,
  ce as __wbg_setheaders_3b47c898e8de6d44,
  se as __wbg_setheaders_834c0bdb6a8949ad,
  ue as __wbg_setmethod_3c5280fe5d890842,
  fe as __wbg_setredirect_40e6a7f717a2f86a,
  ae as __wbg_setsignal_75b21ef3a81de905,
  ge as __wbg_setstatus_51b4fc011091cbb3,
  be as __wbg_static_accessor_GLOBAL_88a902d13a557d07,
  de as __wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0,
  we as __wbg_static_accessor_SELF_37c5d418e4bf5819,
  le as __wbg_static_accessor_WINDOW_5de37043a91a9c40,
  pe as __wbg_status_f6360336ca686bf0,
  ye as __wbg_then_44b73946d2fb3e7d,
  xe as __wbg_then_48b406749878a531,
  he as __wbg_toString_c813bbd34d063839,
  me as __wbg_url_8f9653b899456042,
  Re as __wbg_view_fd8a56e8983f448d,
  Fe as __wbg_webSocket_4a16d2250705e19d,
  Se as __wbindgen_bigint_from_u64,
  je as __wbindgen_boolean_get,
  Ee as __wbindgen_cb_drop,
  ke as __wbindgen_closure_wrapper1084,
  ze as __wbindgen_debug_string,
  Ie as __wbindgen_error_new,
  Oe as __wbindgen_in,
  Te as __wbindgen_init_externref_table,
  Ae as __wbindgen_is_function,
  qe as __wbindgen_is_null,
  Le as __wbindgen_is_object,
  Me as __wbindgen_is_string,
  Ue as __wbindgen_is_undefined,
  De as __wbindgen_jsval_loose_eq,
  Ce as __wbindgen_memory,
  We as __wbindgen_number_get,
  Be as __wbindgen_number_new,
  $e as __wbindgen_string_get,
  Ne as __wbindgen_string_new,
  Pe as __wbindgen_throw,
  Ke as default,
  O as fetch,
  Ve as wasmModule
};
//# sourceMappingURL=shim.js.map
