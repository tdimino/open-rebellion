
void * __thiscall FUN_005a0e70(void *this,uint param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651e53;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0059e880(this);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066bb58;
  FUN_005c0e70();
  FUN_005c0e60();
  FUN_005c0e80();
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 1;
    if (puVar1 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar1);
    }
    local_4 = (uint)local_4._1_3_ << 8;
  }
  iVar2 = FUN_005a6450(DAT_006bc470,param_1);
  FUN_005c5ca0((void *)((int)this + 0x40),iVar2);
  ExceptionList = local_c;
  return this;
}

