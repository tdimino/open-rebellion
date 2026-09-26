
undefined4 * __fastcall FUN_0052d7b0(undefined4 *param_1)

{
  uint uVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645025;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5ea0(param_1);
  local_4 = 0;
  FUN_00520570(param_1 + 0x16);
  local_4._0_1_ = 1;
  FUN_00520570(param_1 + 0x18);
  local_4._0_1_ = 2;
  FUN_00520570(param_1 + 0x1a);
  local_4._0_1_ = 3;
  FUN_005401b0(param_1 + 0x33);
  local_4._0_1_ = 4;
  FUN_005401b0(param_1 + 0x35);
  local_4 = CONCAT31(local_4._1_3_,5);
  *param_1 = &PTR_FUN_00660af8;
  param_1[0xc] = &PTR_FUN_00660af4;
  uVar1 = FUN_00540420((int)param_1);
  uVar2 = 0;
  if (uVar1 != 0) {
    uVar2 = FUN_00533300((int)param_1);
  }
  FUN_00540430(param_1,uVar2);
  ExceptionList = local_c;
  return param_1;
}

