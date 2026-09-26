
void * __thiscall FUN_004f4000(void *this,void *param_1)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f63f;
  local_c = ExceptionList;
  uVar3 = 1;
  if (this != param_1) {
    ExceptionList = &local_c;
    FUN_005f5540(this);
    iVar1 = thunk_FUN_005f5060((int)param_1);
    do {
      if (iVar1 == 0) break;
      pvVar2 = (void *)FUN_00618b70(0x28);
      if (pvVar2 == (void *)0x0) {
        FUN_00619730();
      }
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_00540ad0(pvVar2,iVar1);
      }
      uVar3 = (uint)(pvVar2 != (void *)0x0);
      local_4 = 0xffffffff;
      if (uVar3 != 0) {
        FUN_004f4270(this,pvVar2);
      }
      iVar1 = *(int *)(iVar1 + 0x10);
    } while (uVar3 != 0);
  }
  *(uint *)((int)this + 0xc) = uVar3;
  ExceptionList = local_c;
  return this;
}

