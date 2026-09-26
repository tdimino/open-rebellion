
void __fastcall FUN_0059cac0(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651a6b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_006037f0(6);
  pvVar1 = (void *)FUN_00618b70(0xa8);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005c5520(pvVar1,*(undefined4 *)(param_1 + 0x1c),0x1e2,0x1c,0x95,0xec,param_1);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0x10c) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_005ffce0(pvVar1,0);
    (**(code **)(**(int **)(param_1 + 0x10c) + 4))(5);
  }
  ExceptionList = local_c;
  return;
}

