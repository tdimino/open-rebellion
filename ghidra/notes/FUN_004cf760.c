
undefined4 __fastcall FUN_004cf760(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  iVar1 = FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),*(uint *)(param_1 + 0x44));
  if (iVar1 == 0) {
    iVar1 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c) + 0xd8);
    if (iVar1 == 0) {
      *(undefined4 *)(param_1 + 0x40) = 0;
      goto LAB_004cf7b2;
    }
  }
  uVar2 = 1;
  *(undefined4 *)(param_1 + 0x40) = *(undefined4 *)(iVar1 + 0x18);
  if (*(int *)(iVar1 + 0x10) != 0) {
    *(undefined4 *)(param_1 + 0x44) = *(undefined4 *)(*(int *)(iVar1 + 0x10) + 0x18);
    return 1;
  }
LAB_004cf7b2:
  *(undefined4 *)(param_1 + 0x44) = 0;
  return uVar2;
}

