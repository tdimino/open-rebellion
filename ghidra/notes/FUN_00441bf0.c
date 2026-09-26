
void __fastcall FUN_00441bf0(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_0062eac6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_LAB_00659b60;
  puVar1 = (undefined4 *)param_1[1];
  local_4 = 1;
  param_1[0x1a] = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
    param_1[1] = 0;
  }
  puVar1 = (undefined4 *)param_1[2];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
    param_1[2] = 0;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004aa830(param_1 + 9);
  local_4 = 0xffffffff;
  FUN_004aa900(param_1 + 6);
  ExceptionList = pvStack_c;
  return;
}

