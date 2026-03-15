
void __fastcall FUN_0041c610(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062b7d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658a88;
  local_4 = 0;
  if ((undefined *)param_1[0xc] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0xc]);
  }
  if ((undefined *)param_1[0xd] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0xd]);
  }
  local_4 = 0xffffffff;
  FUN_005f55f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

