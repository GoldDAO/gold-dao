export const bigintTo32ByteArray = (value: bigint) => {
  const byteArray = new Uint8Array(32);
  for (let i = byteArray.length - 1; i >= 0; i--) {
    byteArray[i] = Number(value & 0xffn);
    value >>= 8n;
  }
  return byteArray.reverse();
};
