
undefined4 * __fastcall FUN_005bbda0(undefined4 *param_1)

{
  uint uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065360b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00610b20(param_1 + 2,0,0,0,0,0,0);
  local_4 = 0;
  *param_1 = 0;
  param_1[1] = 0;
  param_1[8] = 5;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[9] = 1;
  param_1[6] = param_1[6] & 0xffffffd2 | 2;
  uVar1 = FUN_004019e0();
  param_1[1] = uVar1 >> 8 & 1;
  ExceptionList = local_c;
  return param_1;
}

