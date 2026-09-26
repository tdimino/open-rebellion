
uint __thiscall FUN_0052a2c0(void *this,undefined4 param_1)

{
  void *this_00;
  uint *puVar1;
  int *piVar2;
  uint uVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644928;
  local_c = ExceptionList;
  uVar3 = 1;
  ExceptionList = &local_c;
  local_10 = this;
  this_00 = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x54) + 0x18);
  do {
    if (this_00 == (void *)0x0) {
      ExceptionList = local_c;
      return uVar3;
    }
    puVar1 = FUN_00403040(this_00,&local_10);
    local_4 = 0;
    piVar2 = (int *)FUN_00504dc0(puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar3 = 0;
    if (piVar2 != (int *)0x0) {
      uVar3 = (**(code **)(*piVar2 + 0xac))(0x16,param_1);
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  } while (uVar3 != 0);
  ExceptionList = local_c;
  return 0;
}

