
void __thiscall FUN_0060ca00(void *this,int *param_1)

{
  int *this_00;
  int *piVar1;
  void *pvVar2;
  undefined4 uVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656b0b;
  pvStack_c = ExceptionList;
  this_00 = (int *)((int)this + 0xd0);
  ExceptionList = &pvStack_c;
  FUN_005f5b20(this_00);
  if (*(int *)((int)this + 0xe8) != 0) {
    FUN_0060cbf0((int)this);
  }
  for (piVar1 = (int *)(**(code **)(*param_1 + 8))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
    pvVar2 = (void *)FUN_00618b70(0x68);
    uStack_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_0060bac0(pvVar2,(int)piVar1);
    }
    uStack_4 = 0xffffffff;
    FUN_005f59f0(this_00,(int)pvVar2);
  }
  uVar3 = (**(code **)(*this_00 + 8))();
  *(undefined4 *)((int)this + 0xe0) = uVar3;
  ExceptionList = pvStack_c;
  return;
}

