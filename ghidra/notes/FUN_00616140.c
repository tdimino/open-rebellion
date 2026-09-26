
int * __fastcall FUN_00616140(int *param_1)

{
  int iVar1;
  
  if (*(int *)((int)param_1 + *(int *)(*param_1 + 4) + 0x34) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)param_1 + *(int *)(*param_1 + 4) + 0x38));
  }
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  iVar1 = (**(code **)(**(int **)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 4))();
  if (iVar1 == -1) {
    *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) =
         *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) | 2;
  }
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  if (*(int *)(*(int *)(*param_1 + 4) + 0x34 + (int)param_1) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)param_1 + *(int *)(*param_1 + 4) + 0x38));
  }
  return param_1;
}

