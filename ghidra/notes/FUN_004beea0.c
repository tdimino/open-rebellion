
void __thiscall FUN_004beea0(void *this,void *param_1)

{
  undefined4 *puVar1;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063a01b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvStack_10 = this;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&pvStack_10);
  for (; pvStack_10 != (void *)0x0; pvStack_10 = (void *)((int)pvStack_10 + -1)) {
    puVar1 = (undefined4 *)FUN_00618b70(0x2c);
    uStack_4 = 0;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      puVar1 = FUN_004e4d40(puVar1);
    }
    uStack_4 = 0xffffffff;
    FUN_004e4e20(puVar1,param_1);
  }
  ExceptionList = pvStack_c;
  return;
}

