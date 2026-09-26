
undefined4 __fastcall FUN_0049df70(int param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  
  iVar1 = FUN_0049e130(param_1);
  *(int *)(param_1 + 0x18) = iVar1;
  if (iVar1 == 0) {
    if ((*(int *)(param_1 + 0x10) == 0) ||
       (iVar1 = *(int *)(*(int *)(param_1 + 0x10) + 0x10), *(int *)(param_1 + 0x10) = iVar1,
       iVar1 == 0)) {
      iVar1 = thunk_FUN_005f5060(param_1);
      *(int *)(param_1 + 0x10) = iVar1;
    }
    iVar1 = *(int *)(param_1 + 0x10);
    bVar3 = false;
    iVar2 = FUN_005f4960((undefined4 *)(iVar1 + 0x1c));
    while (iVar2 != 1) {
      if (bVar3) goto LAB_0049dff4;
      iVar2 = *(int *)(*(int *)(param_1 + 0x10) + 0x10);
      *(int *)(param_1 + 0x10) = iVar2;
      if (iVar2 == 0) {
        iVar2 = thunk_FUN_005f5060(param_1);
        *(int *)(param_1 + 0x10) = iVar2;
      }
      bVar3 = iVar1 == *(int *)(param_1 + 0x10);
      iVar2 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x10) + 0x1c));
    }
    if (!bVar3) {
      *(undefined4 *)(param_1 + 0x18) = *(undefined4 *)(param_1 + 0x10);
      return *(undefined4 *)(param_1 + 0x10);
    }
LAB_0049dff4:
    *(undefined4 *)(param_1 + 0x18) = 0;
  }
  return *(undefined4 *)(param_1 + 0x18);
}

