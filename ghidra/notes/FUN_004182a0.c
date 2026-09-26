
undefined4 __fastcall FUN_004182a0(int *param_1)

{
  void *this;
  undefined4 *puVar1;
  int iVar2;
  undefined1 local_28 [8];
  undefined4 local_20 [2];
  int local_18;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b0f0;
  local_c = ExceptionList;
  switch(param_1[0x5f]) {
  case 1:
    ExceptionList = &local_c;
    FUN_0051c740(local_20);
    local_4 = 0;
    FUN_0042dd00(param_1 + 0x4e,*param_1);
    param_1[0x29] = param_1[0x53];
    FUN_004fce50(local_20);
    if (local_18 == 1) {
      param_1[1] = param_1[1] | 1;
    }
    FUN_0041bd20((int)param_1);
    this = (void *)FUN_004f3dd0(*param_1,*param_1);
    if (this != (void *)0x0) {
      puVar1 = FUN_0041bde0(this,local_28);
      local_4._0_1_ = 1;
      FUN_00418970(param_1,puVar1);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_00418990(param_1,*(undefined4 *)((int)this + 0x70));
      param_1[99] = 0;
    }
    param_1[0x5f] = 2;
    local_4 = 0xffffffff;
    FUN_0051c780(local_20);
    ExceptionList = local_c;
    return 0;
  case 2:
    ExceptionList = &local_c;
    iVar2 = FUN_00431040(param_1 + 0xb);
    if (iVar2 != 0) {
      param_1[0x5f] = 3;
      ExceptionList = local_c;
      return 0;
    }
    break;
  case 3:
    ExceptionList = &local_c;
    iVar2 = FUN_00430b80(param_1 + 0x11,param_1 + 0xb,param_1);
    if (iVar2 != 0) {
      for (iVar2 = thunk_FUN_005f5060((int)(param_1 + 0x11)); iVar2 != 0;
          iVar2 = *(int *)(iVar2 + 0x10)) {
        param_1[99] = param_1[99] + 10;
      }
      param_1[0x5f] = 4;
      ExceptionList = local_c;
      return 0;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    iVar2 = FUN_004304d0(param_1 + 0x16);
    if (iVar2 != 0) {
      param_1[0x5f] = 5;
      ExceptionList = local_c;
      return 0;
    }
    break;
  case 5:
    ExceptionList = &local_c;
    iVar2 = FUN_004032c0(param_1 + 0x23);
    if (iVar2 != 0) {
      param_1[0x5f] = 6;
      param_1[1] = param_1[1] | 0x80000000;
      ExceptionList = local_c;
      return 0;
    }
    break;
  case 6:
    ExceptionList = &local_c;
    iVar2 = FUN_00418500(param_1);
    if (iVar2 != 0) {
      ExceptionList = local_c;
      return 1;
    }
    break;
  default:
    param_1[0x5f] = 1;
  }
  ExceptionList = local_c;
  return 0;
}

