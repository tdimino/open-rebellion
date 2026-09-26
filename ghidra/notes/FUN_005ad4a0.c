
void __thiscall
FUN_005ad4a0(void *this,undefined1 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  void *pvVar1;
  void *local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652ebb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar1 = (void *)FUN_00618b70(0x220);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    local_14 = *(void **)((int)this + 0x87c);
    pvVar1 = FUN_005ce500(pvVar1,(float)local_14,param_1,param_2,param_3,(uint *)0x0);
  }
  local_4 = 0xffffffff;
  (**(code **)(*(int *)this + 0x2c))(pvVar1,param_4);
  ExceptionList = local_14;
  return;
}

