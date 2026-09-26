
void * __thiscall FUN_005a2280(void *this,int *param_1,uint param_2)

{
  int *piVar1;
  int iVar2;
  undefined4 *puVar3;
  void *this_00;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652123;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_0059e880(this);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066bc98;
  FUN_005c0e70();
  FUN_005c0e60();
  piVar1 = (int *)(**(code **)(*param_1 + 0x10))();
  iVar2 = (**(code **)(*piVar1 + 0x10))();
  FUN_005c5f20((void *)((int)this + 4),iVar2);
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar3 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 1;
    if (puVar3 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar3);
    }
    local_4 = (uint)local_4._1_3_ << 8;
  }
  iVar2 = FUN_005a6340(DAT_006bc470,param_2);
  FUN_005c5f20((void *)((int)this + 4),iVar2);
  FUN_005c0e80();
  this_00 = (void *)FUN_0059ed70();
  FUN_0051c9b0(this_00,iVar2);
  ExceptionList = pvStack_c;
  return this;
}

