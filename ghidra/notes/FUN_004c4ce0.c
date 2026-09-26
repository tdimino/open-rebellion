
undefined4 * __fastcall FUN_004c4ce0(undefined4 *param_1)

{
  uint *this;
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a7b3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(param_1);
  this = param_1 + 9;
  local_4 = 0;
  FUN_005f4950(this,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  *param_1 = &PTR_FUN_0065c538;
  param_1[7] = 0;
  param_1[8] = 0;
  uVar1 = thunk_FUN_004fcee0();
  param_1[0xe] = uVar1;
  param_1[0x10] = 1;
  param_1[10] = 0;
  *(undefined2 *)(param_1 + 0xb) = 0;
  *(undefined2 *)((int)param_1 + 0x2e) = 0;
  *(undefined2 *)(param_1 + 0xc) = 0;
  *(undefined2 *)((int)param_1 + 0x32) = 0;
  uVar1 = FUN_006158b0();
  param_1[0xd] = uVar1;
  *this = *this | 0x10;
  ExceptionList = local_c;
  return param_1;
}

