
undefined4 * __fastcall FUN_0055a100(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064999f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00553e90(param_1);
  local_4 = 0;
  FUN_005401b0(param_1 + 6);
  local_4._0_1_ = 1;
  FUN_005401b0(param_1 + 8);
  local_4._0_1_ = 2;
  FUN_005401b0(param_1 + 10);
  local_4._0_1_ = 3;
  FUN_005401b0(param_1 + 0xc);
  local_4._0_1_ = 4;
  FUN_005401b0(param_1 + 0xe);
  local_4 = CONCAT31(local_4._1_3_,5);
  FUN_005401b0(param_1 + 0x10);
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1d] = 0;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  *param_1 = &PTR_FUN_00662760;
  puVar1 = param_1 + 0x15;
  iVar2 = 3;
  do {
    puVar1[-3] = 0;
    *puVar1 = 0;
    puVar1[5] = 0;
    puVar1 = puVar1 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  ExceptionList = local_c;
  return param_1;
}

