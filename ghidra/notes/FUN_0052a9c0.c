
uint __thiscall FUN_0052a9c0(void *this,void *param_1)

{
  void *this_00;
  uint *puVar1;
  void *this_01;
  int iVar2;
  uint uVar3;
  bool bVar4;
  int local_1c;
  undefined1 local_18 [4];
  undefined1 local_14 [4];
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006449c0;
  local_c = ExceptionList;
  bVar4 = true;
  local_1c = 0;
  ExceptionList = &local_c;
  local_10 = this;
  this_00 = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x54) + 0x18);
  do {
    if (this_00 == (void *)0x0) break;
    puVar1 = FUN_00403040(this_00,local_18);
    local_4 = 0;
    this_01 = (void *)FUN_00504dc0(puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar4 = false;
    if (this_01 != (void *)0x0) {
      puVar1 = FUN_00402d80(this_01,local_14);
      local_4 = 1;
      iVar2 = FUN_0051cab0(puVar1);
      local_4 = 0xffffffff;
      FUN_00619730();
      bVar4 = iVar2 != 0;
      if (bVar4) {
        iVar2 = FUN_0053b860(iVar2);
        local_1c = local_1c + iVar2;
      }
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  } while (bVar4);
  uVar3 = 0;
  if (bVar4) {
    uVar3 = FUN_005296b0(local_10,local_1c,param_1);
  }
  ExceptionList = local_c;
  return uVar3;
}

