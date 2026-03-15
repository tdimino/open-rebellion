
void * FUN_00420de0(void)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b98f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x34);
  if (pvVar1 == (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (pvVar1 != (void *)0x0) {
    pvVar1 = FUN_00420d70(pvVar1,0,0,0,0,0);
    ExceptionList = local_c;
    return pvVar1;
  }
  ExceptionList = local_c;
  return (void *)0x0;
}

