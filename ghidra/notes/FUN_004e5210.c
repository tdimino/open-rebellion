
void __thiscall FUN_004e5210(void *this,void *param_1)

{
  ushort uVar1;
  int *piVar2;
  void *local_4;
  
  local_4 = this;
  uVar1 = FUN_005f50e0((int)this);
  local_4 = (void *)(uint)uVar1;
  FUN_005f4db0(param_1,&local_4);
  for (piVar2 = (int *)thunk_FUN_005f5060((int)this); piVar2 != (int *)0x0;
      piVar2 = (int *)piVar2[4]) {
    (**(code **)(*piVar2 + 8))(param_1);
  }
  return;
}

