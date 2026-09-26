
undefined4 * __fastcall FUN_00476140(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632e19;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(param_1);
  local_4 = 0;
  FUN_004f5510(param_1 + 9);
  local_4._0_1_ = 1;
  FUN_004f5510(param_1 + 0xb);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 0xd);
  local_4._0_1_ = 3;
  FUN_004ece30(param_1 + 0xe);
  local_4._0_1_ = 4;
  FUN_004ece30(param_1 + 0xf);
  local_4._0_1_ = 5;
  FUN_004ece30(param_1 + 0x10);
  local_4._0_1_ = 6;
  FUN_004355d0(param_1 + 0x12);
  local_4._0_1_ = 7;
  FUN_005f4950(param_1 + 0x18,0);
  local_4._0_1_ = 8;
  FUN_005f4950(param_1 + 0x19,0);
  local_4._0_1_ = 9;
  FUN_005f52c0(param_1 + 0x2c);
  local_4 = CONCAT31(local_4._1_3_,10);
  FUN_005f52c0(param_1 + 0x35);
  *param_1 = &PTR_FUN_0065a3b0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  param_1[0x11] = 0;
  param_1[0x27] = 0;
  puVar2 = param_1 + 0x1b;
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  param_1[0x25] = 0;
  param_1[0x26] = 0;
  param_1[0x28] = 0;
  param_1[0x29] = 0;
  param_1[0x2a] = 0;
  param_1[0x2b] = 0;
  param_1[0x2f] = 0;
  param_1[0x30] = 0;
  param_1[0x31] = 0;
  param_1[0x32] = 0;
  param_1[0x33] = 0;
  param_1[0x34] = 0;
  param_1[0x38] = 0;
  param_1[0x39] = 0;
  param_1[0x1a] = 0;
  ExceptionList = local_c;
  return param_1;
}

