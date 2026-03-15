
undefined4 __cdecl FUN_004f3dd0(int param_1,uint param_2)

{
  uint uVar1;
  undefined4 uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063f598;
  local_c = ExceptionList;
  uVar1 = param_2 & 0xffffff | 0xf3000000;
  param_2 = uVar1 ^ ((param_2 != 1) + 0xf ^ uVar1) & 0xffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar2 = FUN_0053ef00(&param_2,param_1);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar2;
}

