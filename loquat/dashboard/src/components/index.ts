export const YES = "Y";
export const NO = "N";
export const protocol = (tcp: boolean): string => {
  return tcp ? "TCP" : "UDP";
};
