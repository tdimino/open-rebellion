
void __fastcall FUN_00617cb0(int *param_1)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  uVar2 = FUN_006184e0(*(int **)(*(int *)(*param_1 + 4) + 4 + (int)param_1));
  while (uVar2 != 0xffffffff) {
    uVar2 = FUN_00619ff0(uVar2);
    if (uVar2 == 0) goto LAB_00617d3d;
    uVar2 = FUN_006183e0(*(int **)(*(int *)(*param_1 + 4) + 4 + (int)param_1));
  }
  iVar1 = *(int *)(*param_1 + 4);
  uVar2 = *(uint *)(iVar1 + 8 + (int)param_1);
  if (*(int *)(iVar1 + 0x34 + (int)param_1) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)param_1 + iVar1 + 0x38));
  }
  *(uint *)((int)param_1 + iVar1 + 8) = uVar2 | 1;
  if (*(int *)((int)param_1 + iVar1 + 0x34) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)param_1 + iVar1 + 0x38));
  }
LAB_00617d3d:
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  return;
}

