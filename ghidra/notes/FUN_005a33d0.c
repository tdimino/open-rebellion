
void * __thiscall FUN_005a33d0(void *this,uint param_1)

{
  uint uVar1;
  void *pvVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  int iVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065233e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0059e880(this);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  *(undefined ***)this = &PTR_FUN_0066bc48;
  FUN_005c0e70();
  *(uint *)((int)this + 0x54) = param_1;
  pvVar2 = (void *)FUN_0059ed70();
  uVar3 = FUN_0059f5d0(pvVar2,param_1);
  *(undefined4 *)((int)this + 0x58) = uVar3;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar4 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 1;
    if (puVar4 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar4);
    }
    local_4._0_1_ = 0;
  }
  iVar5 = FUN_005a6460(DAT_006bc470,*(uint *)((int)this + 0x58));
  *(int *)((int)this + 0x5c) = iVar5;
  uVar1 = *(uint *)(iVar5 + 0x70);
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar4 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 2;
    if (puVar4 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar4);
    }
    local_4._0_1_ = 0;
  }
  iVar5 = FUN_005a6340(DAT_006bc470,uVar1);
  iVar6 = FUN_0059ed70();
  FUN_0059ef60(iVar6);
  pvVar2 = (void *)FUN_0059ed70();
  FUN_0051c9b0(pvVar2,iVar5);
  ExceptionList = local_c;
  return this;
}

