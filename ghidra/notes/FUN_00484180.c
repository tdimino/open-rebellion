
undefined4 * __fastcall FUN_00484180(undefined4 *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006345ee;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ecc00(param_1);
  local_4 = 0;
  FUN_005f4950(param_1 + 9,0);
  local_4._0_1_ = 1;
  FUN_0042dbe0(param_1 + 10);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_004ece30(param_1 + 0xb);
  param_1[8] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x10] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  param_1[7] = 0;
  *param_1 = &PTR_FUN_0065aa50;
  puVar2 = param_1 + 0x11;
  for (iVar1 = 0x1e; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  ExceptionList = local_c;
  return param_1;
}

