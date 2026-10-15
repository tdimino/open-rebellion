
uint __fastcall FUN_0051ddb0(int param_1)

{
  uint uVar1;
  
  uVar1 = (uint)(*(int *)(param_1 + 0xc0) != 0);
  if (*(int *)(param_1 + 0xc0) != 0) {
    uVar1 = FUN_00516d50();
  }
  *(undefined4 *)(param_1 + 0x28) = *(undefined4 *)(param_1 + 0xc);
  return uVar1;
}

