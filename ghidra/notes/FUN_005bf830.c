
undefined4 * __fastcall FUN_005bf830(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065382e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005a7500(param_1);
  *param_1 = &PTR_LAB_0066c880;
  local_4 = 0;
  param_1[1] = 0;
  param_1[0x226] = 0x41200000;
  FUN_00597170();
  FUN_00597190(2000);
  puVar1 = (undefined4 *)FUN_00618b70(0xb0);
  local_4._0_1_ = 1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005c7cb0(puVar1);
    *puVar1 = &PTR_FUN_0066c7a0;
    puVar1[0x12] = 0;
    puVar1[0x13] = 0;
    puVar1[0x14] = 0x3f800000;
    puVar1[0x15] = 0xbf800000;
  }
  local_4._0_1_ = 0;
  param_1[2] = puVar1;
  puVar1 = (undefined4 *)FUN_00618b70(0xb0);
  local_4 = CONCAT31(local_4._1_3_,2);
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005c7cb0(puVar1);
    *puVar1 = &PTR_FUN_0066c778;
    puVar1[0x12] = 0;
    puVar1[0x13] = 0;
    puVar1[0x14] = 0xbf800000;
    puVar1[0x15] = 0xbf800000;
  }
  param_1[3] = puVar1;
  param_1[0x21a] = 0;
  param_1[0x21b] = 0;
  param_1[0x21c] = 0;
  param_1[0x21d] = 0;
  param_1[0x22b] = 3;
  ExceptionList = local_c;
  return param_1;
}

