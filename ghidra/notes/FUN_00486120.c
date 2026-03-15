
undefined4 * __fastcall FUN_00486120(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006348a3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00488460(param_1);
  local_4 = 0;
  FUN_004bee50(param_1 + 0x17);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004888f0(param_1 + 0x1b);
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  param_1[0x30] = 0;
  param_1[0x16] = 0;
  *param_1 = &PTR_FUN_0065ad70;
  ExceptionList = local_c;
  return param_1;
}

