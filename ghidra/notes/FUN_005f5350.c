
void __thiscall FUN_005f5350(void *this,void *param_1)

{
  undefined4 *puVar1;
  undefined4 *this_00;
  void *pvVar2;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065579b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvStack_10 = this;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&pvStack_10);
  pvVar2 = (void *)0x0;
  if (pvStack_10 != (void *)0x0) {
    do {
      puVar1 = (undefined4 *)FUN_00618b70(0x1c);
      this_00 = (undefined4 *)0x0;
      uStack_4 = 0;
      if (puVar1 != (undefined4 *)0x0) {
        this_00 = FUN_005f5590(puVar1);
      }
      uStack_4 = 0xffffffff;
      FUN_005f5610(this_00,param_1);
      FUN_005f5440(this,this_00);
      pvVar2 = (void *)((int)pvVar2 + 1);
    } while (pvVar2 < pvStack_10);
  }
  ExceptionList = pvStack_c;
  return;
}

