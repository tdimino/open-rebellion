
void * __thiscall FUN_005a1110(void *this,uint param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  void *this_00;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651ee3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0059e880(this);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066bba8;
  FUN_005c0e70();
  FUN_005c0e60();
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
  iVar2 = FUN_005a6340(DAT_006bc470,param_1);
  FUN_005c5f20((void *)((int)this + 0x28),iVar2);
  iVar3 = FUN_0059ed70();
  FUN_0059ef60(iVar3);
  this_00 = (void *)FUN_0059ed70();
  FUN_0051c9b0(this_00,iVar2);
  iVar2 = FUN_005aaf90();
  if (*(int *)(iVar2 + 0x8c8) == 0) {
    FUN_005a11e0((int)this);
  }
  ExceptionList = local_c;
  return this;
}

