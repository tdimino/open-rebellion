
undefined4 * __fastcall FUN_0049c780(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637021;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950(param_1 + 1,0);
  local_4 = 0;
  FUN_004c5910(param_1 + 2);
  local_4._0_1_ = 1;
  FUN_004c5450(param_1 + 8);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_0042dc10(param_1 + 0xd);
  *param_1 = 0;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x1d] = 0;
  puVar2 = param_1 + 0x20;
  for (iVar1 = 0x1e; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  param_1[0x3e] = 0;
  ExceptionList = local_c;
  return param_1;
}

