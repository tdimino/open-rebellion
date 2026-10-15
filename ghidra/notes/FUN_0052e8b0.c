
undefined4 __fastcall FUN_0052e8b0(void *param_1)

{
  undefined4 uVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645278;
  local_c = ExceptionList;
  local_14[0] = 0x10;
  local_14[1] = 0x40;
  local_4 = 0;
  ExceptionList = &local_c;
  uVar1 = FUN_004f6010(param_1,local_14,1);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

