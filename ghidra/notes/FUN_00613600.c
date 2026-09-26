
void __fastcall FUN_00613600(undefined4 *param_1)

{
  int *piVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00657023;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066e3e0;
  local_4 = 1;
  if ((param_1[0xd] != 0) && (param_1[0x15] == 1)) {
    FUN_00613e00((int)param_1);
  }
  if (param_1[0xd] != 0) {
    FUN_00613c00((int)param_1);
    (**(code **)(*(int *)param_1[0xd] + 8))((int *)param_1[0xd]);
    param_1[0xd] = 0;
    param_1[0x14] = 0;
  }
  piVar1 = (int *)param_1[0x1b];
  if ((piVar1 != (int *)0x0) &&
     (*(int *)(*(int *)(*(int *)(*piVar1 + 4) + 4 + (int)piVar1) + 0x4c) != -1)) {
    FUN_006168a0(piVar1);
    param_1[0x1b] = 0;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00613ef0(param_1 + 0x1f);
  local_4 = 0xffffffff;
  FUN_005f5c50(param_1);
  ExceptionList = pvStack_c;
  return;
}

