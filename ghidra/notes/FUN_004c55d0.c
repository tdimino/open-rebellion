
void __thiscall FUN_004c55d0(void *this,void *param_1)

{
  void *pvVar1;
  ushort uVar2;
  int *piVar3;
  uint *puVar4;
  undefined4 *puVar5;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a8e8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_14);
  pvVar1 = param_1;
  local_4 = 0;
  FUN_005f4db0(param_1,(int)this + 8);
  uVar2 = FUN_005f50e0((int)this);
  local_10 = (uint)uVar2;
  FUN_005f4db0(pvVar1,&local_10);
  for (piVar3 = (int *)thunk_FUN_005f5060((int)this); piVar3 != (int *)0x0;
      piVar3 = (int *)piVar3[4]) {
    (**(code **)(*piVar3 + 0x10))(pvVar1);
  }
  if (*(void **)((int)this + 0x10) == (void *)0x0) {
    param_1 = (void *)0x0;
    FUN_005f4db0(pvVar1,&param_1);
  }
  else {
    puVar4 = FUN_004025b0(*(void **)((int)this + 0x10),(uint *)&param_1);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_14,puVar4);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    param_1 = (void *)(local_14 >> 0x18);
    FUN_005f4db0(pvVar1,&param_1);
    FUN_004ecea0(&local_14,pvVar1);
  }
  if (*(void **)((int)this + 0xc) == (void *)0x0) {
    param_1 = (void *)0x0;
    FUN_005f4db0(pvVar1,&param_1);
  }
  else {
    puVar5 = FUN_00403040(*(void **)((int)this + 0xc),&param_1);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_14,puVar5);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    param_1 = (void *)(local_14 >> 0x18);
    FUN_005f4db0(pvVar1,&param_1);
    FUN_004ecea0(&local_14,pvVar1);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

