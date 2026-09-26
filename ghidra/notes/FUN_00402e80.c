
void __thiscall FUN_00402e80(void *this,int *param_1)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00629400;
  local_c = ExceptionList;
  local_4 = 0;
  iVar1 = *param_1;
  ExceptionList = &local_c;
  if ((iVar1 == 0x3c000001) || (ExceptionList = &local_c, FUN_00619730(), iVar1 == 0x3c000005)) {
    bVar2 = true;
  }
  else {
    bVar2 = false;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    uVar3 = *(uint *)((int)this + 0x34) | 0x10000000;
  }
  else {
    iVar1 = *param_1;
    local_4 = 1;
    if ((iVar1 == 0x3c000002) || (FUN_00619730(), iVar1 == 0x3c000006)) {
      bVar2 = true;
    }
    else {
      bVar2 = false;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      uVar3 = *(uint *)((int)this + 0x34) | 0x20000000;
    }
    else {
      iVar1 = *param_1;
      local_4 = 2;
      if ((iVar1 == 0x3c000003) || (FUN_00619730(), iVar1 == 0x3c000007)) {
        bVar2 = true;
      }
      else {
        bVar2 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar2) {
        uVar3 = *(uint *)((int)this + 0x34) | 0x40000000;
      }
      else {
        iVar1 = *param_1;
        local_4 = 3;
        if ((iVar1 == 0x3c000004) || (FUN_00619730(), iVar1 == 0x3c000008)) {
          bVar2 = true;
        }
        else {
          bVar2 = false;
        }
        local_4 = 0xffffffff;
        FUN_00619730();
        if (!bVar2) {
          ExceptionList = local_c;
          return;
        }
        uVar3 = *(uint *)((int)this + 0x34) | 0x80000000;
      }
    }
  }
  *(uint *)((int)this + 0x34) = uVar3;
  ExceptionList = local_c;
  return;
}

