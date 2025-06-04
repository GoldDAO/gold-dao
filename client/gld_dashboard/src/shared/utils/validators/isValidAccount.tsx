export const ICP_ADDRESS_LENGTH = 64;

export const isValidAccount = (address: string): boolean => {
  return (
    /^[0-9a-fA-F]+$/.test(address) && ICP_ADDRESS_LENGTH === address.length
  );
};
