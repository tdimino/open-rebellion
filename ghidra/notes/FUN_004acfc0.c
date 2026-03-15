
void __fastcall FUN_004acfc0(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00638748;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065c058;
  puVar1 = (undefined4 *)param_1[0x2e];
  local_4 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x2f];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4 = 0xffffffff;
  FUN_0060e250(param_1);
  ExceptionList = local_c;
  return;
}

