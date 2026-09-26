
void * __thiscall FUN_005d1e40(void *this,int param_1)

{
  int *piVar1;
  void *pvVar2;
  int iVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006545bb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)this + 4))();
  for (piVar1 = (int *)thunk_FUN_005f5060(param_1); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    pvVar2 = (void *)FUN_00618b70(0x20);
    uStack_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      iVar3 = (**(code **)(*piVar1 + 0x10))();
      pvVar2 = FUN_005e9cc0(pvVar2,iVar3);
    }
    uStack_4 = 0xffffffff;
    (**(code **)(*(int *)this + 0xc))(pvVar2);
  }
  ExceptionList = pvStack_c;
  return this;
}

