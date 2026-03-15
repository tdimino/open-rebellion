
undefined4 * __fastcall FUN_005a6290(undefined4 *param_1)

{
  uint uVar1;
  uint *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652821;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f52c0(param_1 + 5);
  local_4 = 0;
  FUN_005f52c0(param_1 + 8);
  local_4._0_1_ = 1;
  FUN_005f52c0(param_1 + 0xb);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005f52c0(param_1 + 0xe);
  *param_1 = &PTR_FUN_0066bed8;
  param_1[0x11] = 0;
  uVar1 = 0;
  puVar2 = param_1 + 3;
  do {
    puVar2[-2] = uVar1;
    *puVar2 = uVar1;
    uVar1 = uVar1 + 1000000;
    puVar2 = puVar2 + 1;
  } while (uVar1 < 2000000);
  ExceptionList = local_c;
  return param_1;
}

