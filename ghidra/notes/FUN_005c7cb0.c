
undefined4 * __fastcall FUN_005c7cb0(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined1 local_4;
  undefined3 uStack_3;
  
  puStack_8 = &LAB_00653e7f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  param_1[1] = &PTR_FUN_0066bacc;
  param_1[2] = 0;
  param_1[3] = &PTR_FUN_0066bac8;
  param_1[4] = 0;
  local_4 = 1;
  uStack_3 = 0;
  param_1[0x12] = 0;
  param_1[0x13] = 0;
  param_1[0x14] = 0;
  param_1[0x15] = 0xbf800000;
  FUN_005f52c0(param_1 + 0x16);
  param_1[0x16] = &PTR_FUN_0066bdd0;
  local_4 = 2;
  FUN_005f52c0(param_1 + 0x19);
  param_1[0x19] = &PTR_FUN_0066bdd0;
  local_4 = 3;
  FUN_005f52c0(param_1 + 0x1c);
  param_1[0x1c] = &PTR_LAB_0066c390;
  local_4 = 4;
  FUN_005f4ed0(param_1 + 0x22);
  param_1[0x22] = &PTR_FUN_0066cc28;
  local_4 = 5;
  FUN_005f4ed0(param_1 + 0x24);
  param_1[0x24] = &PTR_FUN_0066cc38;
  local_4 = 6;
  FUN_005f4ed0(param_1 + 0x26);
  param_1[0x26] = &PTR_FUN_0066cc38;
  local_4 = 7;
  FUN_005f4ed0(param_1 + 0x28);
  param_1[0x28] = &PTR_FUN_0066cc38;
  _local_4 = CONCAT31(uStack_3,8);
  FUN_005f4ed0(param_1 + 0x2a);
  param_1[0x2a] = &PTR_FUN_0066cc38;
  *param_1 = &PTR_FUN_0066cc48;
  param_1[8] = 0;
  param_1[5] = 1;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  param_1[7] = 0;
  param_1[9] = 0x3ea8f5c3;
  param_1[0x21] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x11] = 0;
  param_1[0x10] = 0;
  param_1[6] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  ExceptionList = local_c;
  return param_1;
}

