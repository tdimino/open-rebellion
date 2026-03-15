
void __fastcall FUN_004093f0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00629bc6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658644;
  param_1[2] = 0;
  param_1[1] = 1;
  local_4 = 1;
  if (DAT_006b1218 != (undefined4 *)0x0) {
    (**(code **)*DAT_006b1218)(1);
    DAT_006b1218 = (undefined4 *)0x0;
  }
  FUN_005975a0();
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

