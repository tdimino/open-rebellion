
void __thiscall FUN_00601380(void *this,int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006560eb;
  local_c = ExceptionList;
  if ((param_1 != 0) && (*(int *)(param_1 + 0x18) != 0)) {
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x20);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_0060f180(pvVar1,*(undefined4 *)(param_1 + 0x18));
    }
    local_4 = 0xffffffff;
    if (pvVar1 != (void *)0x0) {
      *(int *)((int)pvVar1 + 0x1c) = param_1;
      FUN_005f4f10((void *)((int)this + 0x74),(int)pvVar1);
    }
  }
  ExceptionList = local_c;
  return;
}

