
undefined4 __fastcall FUN_00487c20(int param_1)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  if (*(int *)(param_1 + 0xc0) != 0) {
    uVar1 = *(undefined4 *)(*(int *)(param_1 + 0xc0) + 0x10);
  }
  return uVar1;
}

