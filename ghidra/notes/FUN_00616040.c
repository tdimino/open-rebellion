
void __fastcall FUN_00616040(int *param_1)

{
  int iVar1;
  
  *(undefined4 *)(*(int *)(*param_1 + 4) + 0x30 + (int)param_1) = 0;
  if ((*(uint *)((int)param_1 + *(int *)(*param_1 + 4) + 0x24) & 0x2000) != 0) {
    iVar1 = (**(code **)(**(int **)((int)param_1 + *(int *)(*param_1 + 4) + 4) + 4))();
    if (iVar1 == -1) {
      *(undefined4 *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) = 6;
    }
  }
  if ((*(uint *)(*(int *)(*param_1 + 4) + 0x24 + (int)param_1) & 0x4000) != 0) {
    iVar1 = FUN_0061b2a0((int *)&DAT_006acca8);
    if (iVar1 == -1) {
      *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) =
           *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) | 2;
    }
    iVar1 = FUN_0061b2a0((int *)&DAT_006accc8);
    if (iVar1 == -1) {
      *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) =
           *(uint *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) | 2;
    }
  }
  iVar1 = *(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1);
  if (*(int *)(iVar1 + 0x30) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x34));
  }
  if (*(int *)(*(int *)(*param_1 + 4) + 0x34 + (int)param_1) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)param_1 + *(int *)(*param_1 + 4) + 0x38));
  }
  return;
}

