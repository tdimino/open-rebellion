
void __thiscall FUN_00609410(void *this,void *param_1)

{
  int *piVar1;
  void *pvVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065680b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f5b20(param_1);
  for (piVar1 = (int *)(**(code **)(**(int **)((int)this + 0xa0) + 8))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
    if ((*(byte *)(piVar1 + 0xf) & 1) != 0) {
      pvVar2 = (void *)FUN_00618b70(0x68);
      uStack_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_0060bac0(pvVar2,(int)piVar1);
      }
      uStack_4 = 0xffffffff;
      FUN_005f59f0(param_1,(int)pvVar2);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

