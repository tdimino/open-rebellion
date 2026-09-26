
int __fastcall FUN_00616d70(int *param_1)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  
  iVar3 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar3 + 0x30) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(iVar3 + 0x34));
  }
  iVar3 = (**(code **)(**(int **)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 0xc))(0,1,1);
  if (iVar3 == -1) {
    iVar1 = *(int *)(*param_1 + 4);
    uVar2 = *(uint *)(iVar1 + 8 + (int)param_1);
    if (*(int *)(iVar1 + 0x34 + (int)param_1) < 0) {
      FUN_00615ee0((LPCRITICAL_SECTION)((int)param_1 + iVar1 + 0x38));
    }
    *(uint *)((int)param_1 + iVar1 + 8) = uVar2 | 2;
    if (*(int *)((int)param_1 + iVar1 + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)param_1 + iVar1 + 0x38));
    }
  }
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  return iVar3;
}

