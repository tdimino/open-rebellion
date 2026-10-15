
undefined4 * __fastcall FUN_0055b2f0(undefined4 *param_1)

{
  uint uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649bb8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005279d0(param_1);
  local_4 = 0;
  *param_1 = &PTR_FUN_006629a0;
  param_1[0xc] = &PTR_LAB_00662998;
  uVar1 = FUN_00540420((int)param_1);
  FUN_00540430(param_1,uVar1);
  ExceptionList = local_c;
  return param_1;
}

