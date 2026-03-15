
void * __thiscall FUN_005a2160(void *this,uint param_1)

{
  int *piVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006520fb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar1 = (int *)FUN_0059ed70();
  if (*piVar1 != 0) {
    pvVar2 = (void *)FUN_00618b70(0x60);
    local_4 = 0;
    if (pvVar2 != (void *)0x0) {
      pvVar2 = FUN_005a34e0(pvVar2,(int *)((int)this + 4),(int *)((int)this + 0x34),
                            (int *)((int)this + 0x1c),param_1);
      ExceptionList = local_c;
      return pvVar2;
    }
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

