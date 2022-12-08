// Auto-generated with deno_bindgen
import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.2/plug.ts"

function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}

function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}

function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as bigint)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}

const url = new URL("../target/debug", import.meta.url)
let uri = url.toString()
if (!uri.endsWith("/")) uri += "/"

let darwin: string | { aarch64: string; x86_64: string } = uri + "libaoc.dylib"

if (url.protocol !== "file:") {
  // Assume that remote assets follow naming scheme
  // for each macOS artifact.
  darwin = {
    aarch64: uri + "libaoc_arm64.dylib",
    x86_64: uri + "libaoc.dylib",
  }
}

const opts = {
  name: "aoc",
  urls: {
    darwin,
    windows: uri + "aoc.dll",
    linux: uri + "libaoc.so",
  },
  policy: CachePolicy.NONE,
}
const _lib = await prepare(opts, {
  disk_free: {
    parameters: ["pointer", "usize"],
    result: "i32",
    nonblocking: false,
  },
  disk_usage: {
    parameters: ["pointer", "usize"],
    result: "i32",
    nonblocking: false,
  },
  find_top_crates: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: false,
  },
  find_top_multi_crates: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: false,
  },
})

export function disk_free(a0: string) {
  const a0_buf = encode(a0)
  const a0_ptr = Deno.UnsafePointer.of(a0_buf)
  let rawResult = _lib.symbols.disk_free(a0_ptr, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function disk_usage(a0: string) {
  const a0_buf = encode(a0)
  const a0_ptr = Deno.UnsafePointer.of(a0_buf)
  let rawResult = _lib.symbols.disk_usage(a0_ptr, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function find_top_crates(a0: string) {
  const a0_buf = encode(a0)
  const a0_ptr = Deno.UnsafePointer.of(a0_buf)
  let rawResult = _lib.symbols.find_top_crates(a0_ptr, a0_buf.byteLength)
  const result = readPointer(rawResult)
  return decode(result)
}
export function find_top_multi_crates(a0: string) {
  const a0_buf = encode(a0)
  const a0_ptr = Deno.UnsafePointer.of(a0_buf)
  let rawResult = _lib.symbols.find_top_multi_crates(a0_ptr, a0_buf.byteLength)
  const result = readPointer(rawResult)
  return decode(result)
}
