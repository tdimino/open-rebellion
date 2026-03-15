
void __thiscall FUN_005cd870(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  void *local_4;
  
  local_4 = this;
  iVar2 = FUN_005cec50((int)this);
  local_4 = (void *)FUN_005cd800((int)this);
  pvVar1 = param_1;
  FUN_005f4db0(param_1,&local_4);
  while (local_4 != (void *)0x0) {
    local_4 = (void *)((int)local_4 + -1);
    param_1 = *(void **)(*(int *)(iVar2 + 0x18) + 0xec);
    FUN_005f4db0(pvVar1,&param_1);
    iVar2 = *(int *)(iVar2 + 0x10);
  }
  return;
}

