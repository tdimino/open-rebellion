
int __fastcall FUN_005006a0(int param_1)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_1 + 100);
  return (uVar1 >> 8 & 0xf) + (uVar1 >> 0xc & 0xf) + (uVar1 >> 4 & 0xf) + (uVar1 >> 0x10 & 0xf) +
         (uVar1 >> 0x14 & 0xf) + (uVar1 & 0xf);
}

