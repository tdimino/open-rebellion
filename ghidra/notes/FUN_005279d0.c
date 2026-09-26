
undefined4 * __fastcall FUN_005279d0(undefined4 *param_1)

{
  uint uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006443f2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5ea0(param_1);
  local_4 = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  param_1[0x18] = 0;
  FUN_0042dbe0(param_1 + 0x19);
  local_4._0_1_ = 1;
  param_1[0x1a] = 0;
  param_1[0x1b] = 0;
  param_1[0x1c] = 0;
  FUN_004ece30(param_1 + 0x1d);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0x1e);
  local_4._0_1_ = 3;
  FUN_004ece30(param_1 + 0x1f);
  local_4._0_1_ = 4;
  FUN_005f35b0(param_1 + 0x20,&DAT_006b120c);
  local_4 = CONCAT31(local_4._1_3_,5);
  *param_1 = &PTR_FUN_0065fbf8;
  param_1[0xc] = &PTR_LAB_0065fbf0;
  uVar1 = FUN_00540420((int)param_1);
  FUN_00540430(param_1,uVar1);
  ExceptionList = local_c;
  return param_1;
}

