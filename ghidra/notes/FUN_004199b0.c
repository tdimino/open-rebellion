
void __fastcall FUN_004199b0(int param_1)

{
  uint uVar1;
  uint uVar2;
  
  uVar2 = *(uint *)(param_1 + 8);
  if (uVar2 == 0) {
    *(uint *)(param_1 + 0xc) = *(uint *)(param_1 + 0x10);
    uVar2 = *(uint *)(param_1 + 0x10) >> 1;
    *(uint *)(param_1 + 0x10) = uVar2;
    if (uVar2 == 0) {
      *(undefined4 *)(param_1 + 0x10) = 0x80;
      return;
    }
  }
  else {
    uVar1 = *(uint *)(param_1 + 0xc);
    while ((uVar2 & uVar1) == 0) {
      *(uint *)(param_1 + 0xc) = uVar1 >> 1;
      if (uVar1 >> 1 == 0) {
        *(undefined4 *)(param_1 + 0xc) = 0x80;
      }
      uVar1 = *(uint *)(param_1 + 0xc);
    }
    *(uint *)(param_1 + 8) = ~*(uint *)(param_1 + 0xc) & uVar2;
  }
  return;
}

