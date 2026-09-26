
void __thiscall FUN_0042fe90(void *this,void *param_1)

{
  ushort uVar1;
  int *piVar2;
  uint local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cb98;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_005f4db0(param_1,(int)this + 8);
  uVar1 = FUN_005f50e0((int)this);
  local_14 = (uint)uVar1;
  FUN_005f4db0(param_1,&local_14);
  for (piVar2 = (int *)thunk_FUN_005f5060((int)this); piVar2 != (int *)0x0;
      piVar2 = (int *)piVar2[4]) {
    (**(code **)(*piVar2 + 0x10))(param_1);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

