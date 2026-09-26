
undefined4 __fastcall FUN_0057eef0(int param_1)

{
  undefined4 uVar1;
  int iVar2;
  int *local_24;
  undefined4 local_20;
  undefined4 local_1c;
  int local_18;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0064e540;
  local_c = ExceptionList;
  local_20 = 0;
  local_24 = (int *)0x0;
  local_1c = 1;
  local_18 = -1;
  local_14[0] = 1;
  local_14[1] = 0x12;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  ExceptionList = &local_c;
  iVar2 = FUN_005057f0((uint *)(param_1 + 0x3c),&local_24,local_14,&local_1c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  if ((iVar2 != 0) && (local_18 == -1)) {
    (**(code **)(*local_24 + 0x1d4))(param_1 + 0x40,&local_20);
  }
  uVar1 = local_20;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar1;
}

