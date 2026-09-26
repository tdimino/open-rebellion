
void __thiscall FUN_0049de40(void *this,void *param_1)

{
  void *pvVar1;
  short sVar2;
  undefined2 extraout_var;
  int *piVar3;
  void *local_4;
  
  pvVar1 = param_1;
  local_4 = this;
  FUN_005f4db0(param_1,(int)this + 0xc);
  sVar2 = FUN_005f50e0((int)this);
  local_4 = (void *)CONCAT22(extraout_var,sVar2);
  FUN_005f4df0(pvVar1,&local_4);
  for (piVar3 = (int *)thunk_FUN_005f5060((int)this); piVar3 != (int *)0x0;
      piVar3 = (int *)piVar3[4]) {
    param_1 = (void *)piVar3[6];
    FUN_005f4db0(pvVar1,&param_1);
    (**(code **)(*piVar3 + 8))(pvVar1);
  }
  param_1 = (void *)0x0;
  if (*(int *)((int)this + 0x10) != 0) {
    param_1 = *(void **)(*(int *)((int)this + 0x10) + 0x18);
  }
  FUN_005f4db0(pvVar1,&param_1);
  param_1 = (void *)0x0;
  if (*(int *)((int)this + 0x14) != 0) {
    param_1 = *(void **)(*(int *)((int)this + 0x14) + 0x18);
  }
  FUN_005f4db0(pvVar1,&param_1);
  param_1 = (void *)0x0;
  if (*(int *)((int)this + 0x18) != 0) {
    param_1 = *(void **)(*(int *)((int)this + 0x18) + 0x18);
  }
  FUN_005f4db0(pvVar1,&param_1);
  return;
}

