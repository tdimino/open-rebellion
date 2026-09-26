
void __thiscall FUN_0051d6c0(void *this,void *param_1)

{
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00643152;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00568a00();
  FUN_00619730();
  *(undefined4 *)((int)this + 0x2c) = 0;
  if (*(undefined4 **)((int)this + 0xc0) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)((int)this + 0xc0))(1);
    *(undefined4 *)((int)this + 0xc0) = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x1b0);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  uStack_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00513240(puVar1);
  }
  uStack_4 = 0xffffffff;
  *(undefined4 **)((int)this + 0xc0) = puVar1;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_00513640(puVar1,param_1);
    FUN_00513650(*(int *)((int)this + 0xc0));
  }
  ExceptionList = pvStack_c;
  return;
}

