
undefined4 * __fastcall FUN_0059e880(undefined4 *param_1)

{
  int iVar1;
  void *pvVar2;
  undefined4 uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651ac2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f52c0(param_1 + 1);
  param_1[1] = &PTR_FUN_0066bdd0;
  local_4 = 0;
  FUN_005f52c0(param_1 + 4);
  param_1[4] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 1;
  FUN_005f52c0(param_1 + 7);
  param_1[7] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 2;
  FUN_005f52c0(param_1 + 10);
  param_1[10] = &PTR_FUN_0066bdd0;
  local_4._0_1_ = 3;
  FUN_005c6150(param_1 + 0xd);
  param_1[0xd] = &PTR_FUN_0066bdb8;
  local_4._0_1_ = 4;
  FUN_005c6150(param_1 + 0x10);
  param_1[0x10] = &PTR_FUN_0066bdb8;
  local_4 = CONCAT31(local_4._1_3_,5);
  *param_1 = &PTR_FUN_0066bd68;
  param_1[0x14] = 0;
  iVar1 = FUN_0059ed70();
  FUN_0059ef60(iVar1);
  uVar3 = 0;
  pvVar2 = (void *)FUN_0059ed70();
  FUN_0051c9b0(pvVar2,uVar3);
  uVar3 = 0;
  pvVar2 = (void *)FUN_0059ed70();
  FUN_0059f050(pvVar2,uVar3);
  ExceptionList = local_c;
  return param_1;
}

