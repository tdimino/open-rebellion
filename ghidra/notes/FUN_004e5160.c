
void __thiscall FUN_004e5160(void *this,void *param_1)

{
  void *pvVar1;
  int *piVar2;
  void *pvStack_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063e2eb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&pvStack_14);
  param_1 = (void *)0x0;
  if (pvStack_14 != (void *)0x0) {
    do {
      puStack_10 = (undefined4 *)FUN_00618b70(0x28);
      uStack_4 = 0;
      if (puStack_10 == (undefined4 *)0x0) {
        piVar2 = (int *)0x0;
      }
      else {
        piVar2 = FUN_004eca40(puStack_10);
      }
      uStack_4 = 0xffffffff;
      if (piVar2 != (int *)0x0) {
        (**(code **)(*piVar2 + 4))(pvVar1);
        FUN_005f5440(this,piVar2);
      }
      param_1 = (void *)((int)param_1 + 1);
    } while (param_1 < pvStack_14);
  }
  ExceptionList = pvStack_c;
  return;
}

