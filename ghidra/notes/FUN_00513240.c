
undefined4 * __fastcall FUN_00513240(undefined4 *param_1)

{
  uint uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642377;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5ea0(param_1);
  local_4 = 0;
  FUN_0051c740(param_1 + 0x16);
  local_4._0_1_ = 1;
  param_1[0x1b] = 0x500;
  param_1[0x1c] = 0;
  param_1[0x1d] = 0;
  FUN_004ece30(param_1 + 0x23);
  local_4._0_1_ = 2;
  FUN_004fd400(param_1 + 0x24);
  local_4._0_1_ = 3;
  param_1[0x2b] = 0;
  param_1[0x2c] = 0;
  param_1[0x2d] = 0;
  param_1[0x2e] = 0;
  param_1[0x2f] = 0;
  param_1[0x30] = 0;
  param_1[0x31] = 0;
  param_1[0x32] = 0;
  param_1[0x33] = 0;
  param_1[0x34] = 0;
  param_1[0x35] = 0;
  param_1[0x36] = 0;
  param_1[0x37] = 0;
  FUN_004f3f70(param_1 + 0x38);
  local_4._0_1_ = 4;
  FUN_004f3f70(param_1 + 0x3c);
  local_4._0_1_ = 5;
  FUN_004f3f70(param_1 + 0x40);
  local_4._0_1_ = 6;
  FUN_004f3f70(param_1 + 0x44);
  local_4._0_1_ = 7;
  FUN_004f3f70(param_1 + 0x48);
  local_4._0_1_ = 8;
  FUN_004f3f70(param_1 + 0x4c);
  local_4._0_1_ = 9;
  FUN_004f3f70(param_1 + 0x50);
  local_4._0_1_ = 10;
  FUN_0042dbe0(param_1 + 0x54);
  local_4._0_1_ = 0xb;
  FUN_004ece30(param_1 + 0x55);
  local_4._0_1_ = 0xc;
  FUN_004fd400(param_1 + 0x57);
  local_4._0_1_ = 0xd;
  FUN_004fd400(param_1 + 0x5e);
  local_4._0_1_ = 0xe;
  FUN_004fd400(param_1 + 0x65);
  local_4 = CONCAT31(local_4._1_3_,0xf);
  *param_1 = &PTR_FUN_0065e938;
  param_1[0xc] = &PTR_FUN_0065e930;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  param_1[0x21] = 0;
  param_1[0x22] = 0;
  FUN_005f5600(param_1,4);
  FUN_0060f5d0(param_1,0);
  uVar1 = FUN_00540420((int)param_1);
  FUN_00540430(param_1,(uint)(uVar1 != 0));
  ExceptionList = local_c;
  return param_1;
}

