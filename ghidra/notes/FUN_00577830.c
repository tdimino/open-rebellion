
void * __fastcall FUN_00577830(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064cd2f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x40);
  if (pvVar1 == (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (pvVar1 != (void *)0x0) {
    pvVar1 = FUN_00577810(pvVar1,param_1);
    ExceptionList = local_c;
    return pvVar1;
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

