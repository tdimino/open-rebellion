
void __thiscall FUN_004bd940(void *this,int param_1)

{
  void *pvVar1;
  uint *puVar2;
  void *pvVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639df0;
  local_c = ExceptionList;
  if (*(int *)((int)this + 0x78) != param_1) {
    ExceptionList = &local_c;
    *(int *)((int)this + 0x78) = param_1;
    local_10 = this;
    for (pvVar1 = (void *)thunk_FUN_005f5060((int)this + 0x3c); pvVar1 != (void *)0x0;
        pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
      puVar2 = FUN_00403040(pvVar1,&local_10);
      local_4 = 0;
      pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x68) + 0x8c),puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x24) == *(int *)((int)this + 0x18))) {
        *(int *)((int)pvVar3 + 0x2c) = param_1;
      }
    }
    for (pvVar1 = (void *)thunk_FUN_005f5060((int)this + 0x44); pvVar1 != (void *)0x0;
        pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
      puVar2 = FUN_00403040(pvVar1,&local_10);
      local_4 = 1;
      pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x68) + 0x8c),puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x24) == *(int *)((int)this + 0x18))) {
        *(int *)((int)pvVar3 + 0x2c) = param_1;
      }
    }
  }
  ExceptionList = local_c;
  return;
}

