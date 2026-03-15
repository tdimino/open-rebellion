
void __cdecl FUN_004013a0(int param_1,int param_2)

{
  undefined1 *in_stack_00000018;
  undefined1 auStack_20 [12];
  undefined4 uStack_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006290b8;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  if (DAT_006be3b8 != 0) {
    in_stack_00000018 = auStack_20;
    ExceptionList = &local_c;
    FUN_005f2f90(auStack_20,(int)&stack0x0000000c);
    FUN_004158e0(param_1,param_2);
  }
  local_4 = 0xffffffff;
  uStack_14 = 0x401402;
  FUN_005f2ff0((undefined4 *)&stack0x0000000c);
  ExceptionList = local_c;
  return;
}

