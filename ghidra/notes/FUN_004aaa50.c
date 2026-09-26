
void __fastcall FUN_004aaa50(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00638376;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065bef0;
  puVar1 = (undefined4 *)param_1[0x47];
  local_4 = 1;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x48];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((undefined *)param_1[0x49] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x49]);
  }
  if ((HDC)param_1[0x4a] != (HDC)0x0) {
    DeleteDC((HDC)param_1[0x4a]);
  }
  for (iVar2 = param_1[0x50]; iVar2 != 0; iVar2 = *(int *)(iVar2 + 8)) {
    puVar1 = *(undefined4 **)(iVar2 + 0x3c);
    if (puVar1 != (undefined4 *)0x0) {
      FUN_00600280((int)puVar1);
      (**(code **)*puVar1)(1);
    }
  }
  if (param_1[0x4d] != 0) {
    KillTimer((HWND)param_1[6],param_1[0x4d]);
    param_1[0x4d] = 0;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004c7f90(param_1 + 0x4e);
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = pvStack_c;
  return;
}

