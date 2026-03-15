
void __fastcall FUN_005ff250(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00655f78;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066ded8;
  puVar1 = (undefined4 *)param_1[0x25];
  local_4 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x26];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

