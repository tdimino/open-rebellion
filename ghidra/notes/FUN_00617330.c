
uint __fastcall FUN_00617330(int *param_1)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = FUN_00617a90(param_1,1);
  if (iVar1 != 0) {
    uVar2 = FUN_00618440(*(int **)(*(int *)(*param_1 + 4) + 4 + (int)param_1));
    if (uVar2 == 0xffffffff) {
      *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) =
           *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) | 1;
    }
    else {
      param_1[2] = param_1[2] + 1;
    }
    iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
    if (*(int *)(iVar1 + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
    }
    if (*(int *)((int)param_1 + *(int *)(*param_1 + 4) + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)param_1 + *(int *)(*param_1 + 4) + 0x38));
    }
    return uVar2;
  }
  return 0xffffffff;
}

