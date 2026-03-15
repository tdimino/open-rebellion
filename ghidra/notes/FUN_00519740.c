
bool __fastcall FUN_00519740(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642c7b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(8);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005f5680(pvVar1,*(int *)(param_1 + 0x68));
  }
  *(void **)(param_1 + 0xac) = pvVar1;
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_0053e280(pvVar1);
  }
  ExceptionList = local_c;
  return pvVar1 != (void *)0x0;
}

