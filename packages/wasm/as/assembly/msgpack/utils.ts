import { Context } from "./Context";

export class BLOCK {
  /** Memory manager info. */
  mmInfo: u32;
}
export const BLOCK_OVERHEAD: usize = offsetof<BLOCK>();
export const BLOCK_MAXSIZE: usize = (1 << 30) - BLOCK_OVERHEAD;
export const E_INDEXOUTOFRANGE = "Index out of range";
export const E_INVALIDLENGTH = "Invalid length";

export function throwIndexOutOfRange(
  context: Context,
  method: string,
  length: u32,
  byteOffset: u32,
  byteLength: u32
): void {
  throw new RangeError(
    context.printWithContext(
      method + ": " +
      E_INDEXOUTOFRANGE +
      "[length: " +
      length.toString() +
      " byteOffset: " +
      byteOffset.toString() +
      " byteLength: " +
      byteLength.toString() +
      "]"
    )
  );
}
