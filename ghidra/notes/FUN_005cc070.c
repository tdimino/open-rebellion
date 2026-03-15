
void * __thiscall FUN_005cc070(void *this,uint param_1,undefined4 param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  void *this_00;
  int *piVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065431e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005c9fb0(this,param_1,param_2);
  *(undefined ***)this = &PTR_FUN_0066bf70;
  *(undefined ***)((int)this + 0x1c) = &PTR_LAB_0066bfe8;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar5 = (undefined4 *)FUN_00618b70(0x48);
    local_4._0_1_ = 1;
    if (puVar5 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar5);
    }
  }
  local_4._0_1_ = 0;
  iVar6 = FUN_005a6340(DAT_006bc470,param_1);
  uVar1 = *(undefined4 *)(iVar6 + 0x30);
  uVar2 = *(undefined4 *)(iVar6 + 0x34);
  uVar3 = *(undefined4 *)(iVar6 + 0x38);
  pvVar4 = *(void **)(iVar6 + 0x3c);
  this_00 = (void *)FUN_00618b70(0x140);
  local_4._0_1_ = 2;
  if (this_00 == (void *)0x0) {
    piVar7 = (int *)0x0;
  }
  else {
    piVar7 = (int *)FUN_005ec310(this_00,param_1,uVar1,uVar2,uVar3,pvVar4);
  }
  *(int **)((int)this + 0x54) = piVar7;
  local_4 = (uint)local_4._1_3_ << 8;
  (**(code **)(*piVar7 + 0x1c))();
  *(undefined4 *)((int)this + 0x3c) = 2;
  *(undefined4 *)((int)this + 0x38) = 2;
  *(undefined4 *)((int)this + 0x58) = 2;
  *(undefined4 *)((int)this + 0x5c) = 2;
  ExceptionList = pvVar4;
  return this;
}

