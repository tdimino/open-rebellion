
void __thiscall FUN_0048a340(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634e7b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x60);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
                    /* WARNING: Load size is inaccurate */
    pvVar1 = FUN_0049a1b0(pvVar1,*this,param_1);
  }
  local_4 = 0xffffffff;
  FUN_0041d7b0((LPARAM)pvVar1);
  ExceptionList = local_c;
  return;
}

