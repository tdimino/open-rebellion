
void __thiscall FUN_005d6f80(void *this,int *param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654a4b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x14);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005eef90(pvVar1,param_1);
  }
  local_4 = 0xffffffff;
  FUN_005f6700((void *)((int)this + 0x118),(int)pvVar1);
  ExceptionList = local_c;
  return;
}

