
void __thiscall FUN_0042eee0(void *this,void *param_1)

{
  void *pvVar1;
  ushort uVar2;
  int *piVar3;
  void *pvStack_4;
  
  pvVar1 = param_1;
  pvStack_4 = this;
  FUN_005f4db0(param_1,(int)this + 0xc);
  uVar2 = FUN_005f50e0((int)this);
  param_1 = (void *)(uint)uVar2;
  FUN_005f4db0(pvVar1,&param_1);
  for (piVar3 = (int *)thunk_FUN_005f5060((int)this); piVar3 != (int *)0x0;
      piVar3 = (int *)piVar3[4]) {
    pvStack_4 = (void *)(**(code **)(*piVar3 + 0x18))();
    FUN_005f4db0(pvVar1,&pvStack_4);
    (**(code **)(*piVar3 + 8))(pvVar1);
  }
  return;
}

