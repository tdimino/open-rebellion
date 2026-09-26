
undefined4 __fastcall FUN_004a76e0(int *param_1)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  
  piVar1 = FUN_004a25c0(param_1);
  if (piVar1 == (int *)0x0) {
    return 0;
  }
  if (*(int *)(param_1[0x52] + 0x9c) == 1) {
    uVar3 = 1;
  }
  else if (*(int *)(param_1[0x52] + 0x9c) == 2) {
    uVar3 = 2;
  }
  else {
    uVar3 = 0;
  }
  iVar2 = FUN_004feea0(piVar1,uVar3,3);
  if (iVar2 == 0) {
    if (*(int *)(param_1[0x52] + 0x9c) == 1) {
      uVar3 = 2;
    }
    else if (*(int *)(param_1[0x52] + 0x9c) == 2) {
      uVar3 = 1;
    }
    else {
      uVar3 = 0;
    }
    iVar2 = FUN_004feea0(piVar1,uVar3,3);
    if (iVar2 == 0) {
      uVar3 = (uint)piVar1[9] >> 6 & 3;
    }
  }
  if (uVar3 != 1) {
    if (uVar3 != 2) {
      return 0x2d12;
    }
    return 0x2d11;
  }
  return 0x2d10;
}

