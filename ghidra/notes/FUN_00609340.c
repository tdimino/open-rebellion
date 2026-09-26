
void __thiscall FUN_00609340(void *this,undefined4 param_1)

{
  void *pvVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006567eb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar1 = (void *)FUN_00618b70(0x10);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_0060a790(pvVar1,param_1);
  }
  local_4 = 0xffffffff;
  iVar2 = (**(code **)(**(int **)((int)this + 0xa0) + 8))();
  while (iVar2 != 0) {
    FUN_005f5ac0(*(void **)((int)this + 0xa0),iVar2);
    FUN_005f59f0(pvVar1,iVar2);
    iVar2 = (**(code **)(**(int **)((int)this + 0xa0) + 8))();
  }
  if (*(undefined4 **)((int)this + 0xa0) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)((int)this + 0xa0))(1);
  }
  *(void **)((int)this + 0xa0) = pvVar1;
  FUN_00609ae0(this);
  FUN_00609a00(this);
  if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  ExceptionList = pvStack_c;
  return;
}

