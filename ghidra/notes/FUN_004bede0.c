
int __fastcall FUN_004bede0(int param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  
  iVar4 = 0;
  uVar3 = 0;
  if ((*(int *)(param_1 + 0x14) == 0) ||
     (iVar1 = *(int *)(*(int *)(param_1 + 0x14) + 0x10), *(int *)(param_1 + 0x14) = iVar1,
     iVar1 == 0)) {
    iVar1 = thunk_FUN_005f5060(param_1);
    *(int *)(param_1 + 0x14) = iVar1;
  }
  iVar1 = *(int *)(param_1 + 0x14);
  while (iVar1 != 0) {
    iVar2 = FUN_005f4960((undefined4 *)(iVar1 + 0x1c));
    if ((iVar2 == 1) && (uVar3 < *(uint *)(iVar1 + 0x34))) {
      uVar3 = *(uint *)(iVar1 + 0x34);
      iVar4 = iVar1;
    }
    iVar1 = *(int *)(iVar1 + 0x10);
    if (iVar1 == 0) {
      iVar1 = thunk_FUN_005f5060(param_1);
    }
    if (iVar1 == *(int *)(param_1 + 0x14)) {
      iVar1 = 0;
    }
  }
  return iVar4;
}

