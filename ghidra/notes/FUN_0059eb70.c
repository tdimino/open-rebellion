
undefined4 * __fastcall FUN_0059eb70(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651b8d;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x10] = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  param_1[0x13] = 0;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  param_1[0x1b] = 0;
  param_1[0x1c] = 0;
  param_1[0x1d] = 0;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  param_1[0x21] = 0;
  param_1[0x22] = 0;
  param_1[0x23] = 0;
  param_1[0x24] = 0;
  param_1[0x25] = 0;
  param_1[0x26] = 0;
  param_1[0x27] = 0;
  param_1[0x28] = 0;
  param_1[0x29] = 0;
  param_1[0x2a] = 0;
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
  FUN_005f5df0(param_1 + 0x37);
  local_4 = 0;
  FUN_005f5df0(param_1 + 0x3b);
  local_4._0_1_ = 1;
  FUN_005f5df0(param_1 + 0x3f);
  local_4._0_1_ = 2;
  FUN_00618c80(param_1 + 0x45,0xc,4,FUN_005a1750);
  param_1[0x61] = 0;
  local_4._0_1_ = 3;
  DAT_006bc468 = param_1;
  *param_1 = 1;
  puVar1 = (undefined4 *)FUN_00618b70(0x54);
  local_4._0_1_ = 4;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006bc464 = (undefined4 *)0x0;
  }
  else {
    DAT_006bc464 = FUN_005a0c80(puVar1);
  }
  local_4 = CONCAT31(local_4._1_3_,3);
  param_1[0x61] = 0;
  FUN_0059ef60((int)param_1);
  FUN_0051c9b0(param_1,0);
  FUN_0059f050(param_1,0);
  param_1[0x43] = 0;
  param_1[0x44] = 0;
  puVar1 = param_1 + 0x59;
  for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar1 = 0;
    puVar1 = puVar1 + 1;
  }
  param_1[0x55] = 0;
  param_1[0x56] = 0;
  param_1[0x57] = 0;
  param_1[0x58] = 0;
  ExceptionList = local_c;
  return param_1;
}

