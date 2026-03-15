
undefined4 __fastcall FUN_0051df30(int param_1)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  undefined3 extraout_var;
  int iVar4;
  undefined4 uVar5;
  
  if (((*(int *)(param_1 + 0xa8) == 0) || (*(int *)(param_1 + 0xac) == 0)) ||
     (*(int *)(param_1 + 0xb0) == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = *(int *)(param_1 + 0xa4) != 0;
  }
  if (!bVar1) {
    return 1;
  }
  iVar4 = 0;
  iVar2 = (**(code **)(**(int **)(param_1 + 0xb0) + 0x1c))();
  if (iVar2 != 0) {
    FUN_0051ebb0();
    iVar4 = FUN_0054ec70(*(int **)(param_1 + 0xb0));
  }
  if (iVar4 == 0) {
    iVar2 = (**(code **)(**(int **)(param_1 + 0xa8) + 0x1c))();
    if (iVar2 != 0) {
      FUN_0051ebb0();
      iVar4 = FUN_0054ec70(*(int **)(param_1 + 0xa8));
    }
    if ((iVar4 == 0) && (iVar2 = (**(code **)(**(int **)(param_1 + 0xac) + 0x1c))(), iVar2 != 0)) {
      FUN_0051ebb0();
      iVar4 = FUN_0054ec70(*(int **)(param_1 + 0xac));
    }
  }
  if (iVar4 == 0) {
    piVar3 = (int *)thunk_FUN_005f5060(*(int *)(param_1 + 0xa4));
    if (piVar3 == (int *)0x0) {
      uVar5 = 1;
      goto LAB_0051e02c;
    }
    FUN_00537030(*(void **)(param_1 + 0xa4),piVar3[6]);
    FUN_0051dd20(piVar3);
  }
  uVar5 = 0;
LAB_0051e02c:
  bVar1 = FUN_0051eaf0(param_1);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    return 1;
  }
  return uVar5;
}

