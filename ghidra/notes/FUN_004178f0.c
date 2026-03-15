
undefined4 * __fastcall FUN_004178f0(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062af85;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f4950(param_1 + 1,0);
  local_4 = 0;
  FUN_004ece30(param_1 + 5);
  local_4._0_1_ = 1;
  FUN_004ece30(param_1 + 6);
  local_4._0_1_ = 2;
  FUN_004ece30(param_1 + 7);
  local_4._0_1_ = 3;
  FUN_004ece30(param_1 + 8);
  local_4._0_1_ = 4;
  FUN_004ece30(param_1 + 9);
  local_4._0_1_ = 5;
  FUN_004ece30(param_1 + 10);
  local_4._0_1_ = 6;
  FUN_00430d50(param_1 + 0xb);
  local_4._0_1_ = 7;
  FUN_00430890(param_1 + 0x11);
  local_4._0_1_ = 8;
  FUN_00430240(param_1 + 0x16);
  local_4._0_1_ = 9;
  FUN_0042fd50(param_1 + 0x1e);
  puVar2 = param_1 + 0x23;
  local_4._0_1_ = 10;
  FUN_00403060(puVar2);
  local_4._0_1_ = 0xb;
  FUN_0042f4c0(param_1 + 0x2a);
  local_4._0_1_ = 0xc;
  FUN_0042f4c0(param_1 + 0x30);
  local_4._0_1_ = 0xd;
  FUN_0042f190(param_1 + 0x36);
  local_4._0_1_ = 0xe;
  FUN_0042ee00(param_1 + 0x3b);
  local_4._0_1_ = 0xf;
  FUN_0042ee00(param_1 + 0x41);
  local_4._0_1_ = 0x10;
  FUN_0042ec30(param_1 + 0x47);
  local_4._0_1_ = 0x11;
  FUN_004ece30(param_1 + 0x4c);
  local_4._0_1_ = 0x12;
  FUN_004ece30(param_1 + 0x4d);
  local_4._0_1_ = 0x13;
  FUN_0042dc10(param_1 + 0x4e);
  *param_1 = 0;
  param_1[0x5f] = 0;
  param_1[0x60] = 0;
  param_1[0x5e] = 0;
  param_1[3] = 0x80;
  param_1[4] = 0x40;
  param_1[2] = 0;
  param_1[0x10] = param_1 + 0x4e;
  param_1[0x1b] = param_1 + 0x1e;
  param_1[0x1c] = param_1 + 0x4e;
  param_1[0x1d] = param_1;
  param_1[0x21] = param_1;
  param_1[0x22] = param_1 + 0x16;
  param_1[0x27] = param_1;
  param_1[0x2e] = puVar2;
  param_1[0x2f] = param_1;
  param_1[0x3f] = puVar2;
  param_1[0x40] = param_1;
  param_1[0x3a] = param_1;
  param_1[0x4b] = param_1;
  local_4 = CONCAT31(local_4._1_3_,0x14);
  (**(code **)(param_1[0x30] + 4))();
  (**(code **)(param_1[0x41] + 4))();
  puVar2 = param_1 + 0x61;
  for (iVar1 = 100; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  param_1[0xc5] = 0;
  param_1[0xc6] = 0;
  param_1[199] = 0;
  puVar2 = param_1 + 200;
  for (iVar1 = 0xd; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  param_1[0xd5] = 0;
  param_1[0xd6] = 0;
  param_1[0xd7] = 0;
  param_1[0xd8] = 0;
  puVar2 = param_1 + 0xd9;
  for (iVar1 = 0xc; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  ExceptionList = pvStack_c;
  return param_1;
}

