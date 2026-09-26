
void __thiscall FUN_00433150(void *this,void *param_1)

{
  void *pvVar1;
  ushort uVar2;
  undefined4 *puVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  undefined4 local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d170;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_14);
  pvVar1 = param_1;
  local_4 = 0;
  FUN_0058f0d0(this,param_1);
  FUN_005f4db0(pvVar1,(int)this + 0x20);
  FUN_005f4db0(pvVar1,(int)this + 0x24);
  FUN_005f4db0(pvVar1,(int)this + 0x3c);
  FUN_005f4db0(pvVar1,(int)this + 0x1c);
  FUN_005f4db0(pvVar1,(int)this + 0x30);
  FUN_005f4db0(pvVar1,(int)this + 0x34);
  FUN_004ecea0((void *)((int)this + 0x28),pvVar1);
  FUN_004ecea0((void *)((int)this + 0x2c),pvVar1);
  FUN_005f49a0((void *)((int)this + 0x38),pvVar1);
  uVar2 = FUN_005f50e0((int)this + 0x48);
  local_10 = (uint)uVar2;
  FUN_005f4db0(pvVar1,&local_10);
  if (*(void **)((int)this + 0x44) != (void *)0x0) {
    puVar3 = FUN_00403040(*(void **)((int)this + 0x44),&param_1);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_14,puVar3);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  FUN_004ecea0(&local_14,pvVar1);
  for (piVar4 = (int *)thunk_FUN_005f5060((int)this + 0x48); piVar4 != (int *)0x0;
      piVar4 = (int *)piVar4[4]) {
    (**(code **)(*piVar4 + 0x10))(pvVar1);
  }
  iVar6 = (int)this + 0x50;
  iVar5 = 0x1e;
  do {
    FUN_005f4db0(pvVar1,iVar6);
    iVar6 = iVar6 + 4;
    iVar5 = iVar5 + -1;
  } while (iVar5 != 0);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

