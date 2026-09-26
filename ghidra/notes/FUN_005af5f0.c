
undefined4 * __fastcall FUN_005af5f0(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653049;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5650(param_1 + 1);
  local_4 = 0;
  FUN_005f52c0(param_1 + 0xb);
  param_1[0xb] = &PTR_FUN_0066c380;
  local_4._0_1_ = 1;
  FUN_005f52c0(param_1 + 0xe);
  param_1[0xe] = &PTR_FUN_0066c370;
  local_4._0_1_ = 2;
  FUN_005f52c0(param_1 + 0x13);
  param_1[0x13] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 3;
  FUN_005f52c0(param_1 + 0x16);
  param_1[0x16] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 4;
  FUN_005f52c0(param_1 + 0x19);
  param_1[0x19] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 5;
  FUN_005f52c0(param_1 + 0x1c);
  param_1[0x1c] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 6;
  FUN_00618c80(param_1 + 0x1f,0xc,0x26,FUN_005f2f50);
  local_4._0_1_ = 7;
  *param_1 = 0;
  FUN_005f56b0(param_1 + 1,0xbc614e);
  param_1[3] = 1;
  puVar1 = (undefined4 *)FUN_00618b70(0xb4);
  local_4._0_1_ = 8;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005c7cb0(puVar1);
    *puVar1 = &PTR_FUN_0066c428;
    puVar1[0x2c] = 0;
    puVar1[0x12] = 0;
    puVar1[0x13] = 0;
    puVar1[0x14] = 0x3f800000;
    puVar1[0x15] = 0xbf800000;
  }
  local_4._0_1_ = 7;
  param_1[0x93] = puVar1;
  puVar1 = (undefined4 *)FUN_00618b70(0xb4);
  local_4 = CONCAT31(local_4._1_3_,9);
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005c7cb0(puVar1);
    *puVar1 = &PTR_FUN_0066c3f8;
    puVar1[0x2c] = 0;
    puVar1[0x12] = 0;
    puVar1[0x13] = 0;
    puVar1[0x14] = 0xbf800000;
    puVar1[0x15] = 0xbf800000;
  }
  param_1[0x94] = puVar1;
  puVar1 = param_1 + 0x97;
  iVar2 = 2;
  do {
    puVar1[-2] = 0;
    *puVar1 = 0;
    puVar1[2] = 0;
    puVar1[4] = 0;
    puVar1 = puVar1 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  ExceptionList = local_c;
  return param_1;
}

