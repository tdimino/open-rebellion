
void __fastcall FUN_00421700(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_0062ba96;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00658bb0;
  local_4 = 1;
  if ((undefined *)param_1[0x27] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x27]);
  }
  FUN_00421340();
  puVar1 = (undefined4 *)param_1[0x2c];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_005f2ff0(param_1 + 0x29);
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = local_c;
  return;
}

