
void __fastcall FUN_005c5400(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00653bb8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066ca60;
  puVar1 = (undefined4 *)param_1[0x45];
  local_4 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x46];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4 = 0xffffffff;
  FUN_00602400(param_1);
  ExceptionList = local_c;
  return;
}

