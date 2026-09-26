
undefined4 __thiscall FUN_00568bd0(void *this,void *param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  void *pvVar3;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064b38b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvStack_10 = this;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,(int)this + 8);
  FUN_005f4d90(param_1,&pvStack_10);
  pvVar3 = (void *)0x0;
  if (pvStack_10 != (void *)0x0) {
    do {
      puVar1 = (undefined4 *)FUN_00618b70(0x30);
      piVar2 = (int *)0x0;
      uStack_4 = 0;
      if (puVar1 != (undefined4 *)0x0) {
        piVar2 = FUN_0058ee40(puVar1);
      }
      uStack_4 = 0xffffffff;
      (**(code **)(*piVar2 + 0xc))(param_1);
      FUN_00568d50(this,(int)piVar2);
      pvVar3 = (void *)((int)pvVar3 + 1);
    } while (pvVar3 < pvStack_10);
  }
  ExceptionList = pvStack_c;
  return 1;
}

