
undefined4 * __fastcall FUN_00507130(undefined4 *param_1)

{
  int iVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006415b6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f5ea0(param_1);
  iVar1 = DAT_00661a88 - DAT_00661a84;
  param_1[0x17] = 0;
  param_1[0x16] = iVar1 / 2;
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  local_4 = 0;
  param_1[0x1b] = DAT_00661a84;
  param_1[0x1c] = DAT_00661a88;
  param_1[0x1d] = DAT_00661a88;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  FUN_00558820(param_1 + 0x21);
  local_4 = CONCAT31(local_4._1_3_,1);
  param_1[0x22] = 0;
  *param_1 = &PTR_FUN_0065e640;
  param_1[0xc] = &PTR_FUN_0065e63c;
  uVar2 = FUN_00540420((int)param_1);
  FUN_00540430(param_1,(uint)(uVar2 != 0));
  ExceptionList = local_c;
  return param_1;
}

