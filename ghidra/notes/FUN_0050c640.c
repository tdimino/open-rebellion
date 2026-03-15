
void * __fastcall FUN_0050c640(int *param_1)

{
  void *unaff_retaddr;
  undefined4 local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641d5b;
  pvStack_c = ExceptionList;
  local_10 = (void *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_14);
  local_4 = 1;
  (**(code **)(*param_1 + 0xc))(&local_14);
  FUN_004f26d0(unaff_retaddr,(undefined4 *)&stack0xffffffe8);
  local_14 = 1;
  puStack_8 = (undefined1 *)((uint)puStack_8 & 0xffffff00);
  FUN_00619730();
  ExceptionList = local_10;
  return unaff_retaddr;
}

