
undefined4 * __fastcall FUN_005b82a0(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006534b1;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0066c698;
  param_1[9] = 0;
  param_1[0xb] = 0;
  puVar1 = (undefined4 *)FUN_00618b70(0xc);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005f52c0(puVar1);
    *puVar1 = &PTR_FUN_0066bdd0;
  }
  local_4 = 0xffffffff;
  param_1[7] = puVar1;
  puVar1 = (undefined4 *)FUN_00618b70(0xc);
  local_4 = 1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005f52c0(puVar1);
    *puVar1 = &PTR_FUN_0066bdd0;
  }
  local_4 = 0xffffffff;
  param_1[8] = puVar1;
  puVar1 = (undefined4 *)FUN_00618b70(0xc);
  local_4 = 2;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005f52c0(puVar1);
    *puVar1 = &PTR_FUN_0066bdd0;
  }
  param_1[10] = puVar1;
  param_1[1] = 0;
  param_1[6] = DAT_0066c68c;
  param_1[2] = 0;
  param_1[5] = 0;
  param_1[4] = 0;
  param_1[3] = 0;
  ExceptionList = local_c;
  return param_1;
}

