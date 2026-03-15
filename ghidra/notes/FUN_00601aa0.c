
void __thiscall FUN_00601aa0(void *this,char *param_1)

{
  void *pvVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006561cb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)((int)this + 8) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)((int)this + 8))(1);
    *(undefined4 *)((int)this + 8) = 0;
  }
  if (param_1 != (char *)0x0) {
    pvVar1 = (void *)FUN_00618b70(0xc);
    uStack_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_005f35b0(pvVar1,param_1);
    }
    *(void **)((int)this + 8) = pvVar1;
    if ((*(uint *)((int)this + 0x2c) & 1) == 0) {
      *(undefined4 *)((int)this + 0x1c) = 0;
    }
    if ((*(uint *)((int)this + 0x2c) & 2) == 0) {
      *(undefined4 *)((int)this + 0x20) = 0;
    }
  }
  ExceptionList = pvStack_c;
  return;
}

