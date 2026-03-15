
int __fastcall FUN_004c8630(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar2 = 0;
  iVar1 = FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),*(uint *)(param_1 + 0x44));
  if (iVar1 == 0) {
    iVar1 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c) + 0xd8);
    if (iVar1 != 0) goto LAB_004c8663;
  }
  else {
LAB_004c8663:
    do {
      if (iVar2 != 0) break;
      iVar2 = FUN_00476cf0(iVar1);
      if (iVar2 == 0) {
        iVar1 = *(int *)(iVar1 + 0x10);
      }
    } while (iVar1 != 0);
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x40) = *(undefined4 *)(iVar1 + 0x18);
      if (*(int *)(iVar1 + 0x10) != 0) {
        *(undefined4 *)(param_1 + 0x44) = *(undefined4 *)(*(int *)(iVar1 + 0x10) + 0x18);
        return iVar2;
      }
      goto LAB_004c869f;
    }
  }
  *(undefined4 *)(param_1 + 0x40) = 0;
LAB_004c869f:
  *(undefined4 *)(param_1 + 0x44) = 0;
  return iVar2;
}

