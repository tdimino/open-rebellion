
undefined4 __fastcall FUN_0051e220(void *param_1)

{
  uint uVar1;
  undefined4 local_1c [2];
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643220;
  local_c = ExceptionList;
  uVar1 = *(uint *)((int)param_1 + 0xc);
  ExceptionList = &local_c;
  *(uint *)((int)param_1 + 0x50) = uVar1;
  if ((((*(int *)(*(int *)((int)param_1 + 0xc0) + 0x70) != 0) ||
       (uVar1 == *(int *)((int)param_1 + 0x28) + 1U)) || (uVar1 % 0x28 != 0)) ||
     (*(int *)((int)param_1 + 0x48) == 0)) {
    *(uint *)((int)param_1 + 0x54) = uVar1;
    *(undefined4 *)((int)param_1 + 0x58) = *(undefined4 *)((int)param_1 + 8);
    FUN_005f5fa0(local_14);
    local_4 = 0;
    FUN_0051d5e0(param_1,local_14);
    FUN_005f6010((void *)((int)param_1 + 0x5c),(int)local_14);
    FUN_005f5fa0(local_1c);
    local_4._0_1_ = 1;
    FUN_0051cce0(local_1c);
    FUN_0055bef0(local_1c);
    FUN_005f6010((void *)((int)param_1 + 100),(int)local_1c);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f5fe0(local_1c);
    local_4 = 0xffffffff;
    FUN_005f5fe0(local_14);
    if (*(int *)((int)param_1 + 0x48) == 0) {
      *(undefined4 *)((int)param_1 + 0x48) = *(undefined4 *)((int)param_1 + 0xc);
    }
  }
  ExceptionList = local_c;
  return 1;
}

