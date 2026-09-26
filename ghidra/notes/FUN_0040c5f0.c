
uint __thiscall FUN_0040c5f0(void *this,int *param_1)

{
  int *this_00;
  short sVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  undefined *puVar4;
  uint *puVar5;
  uint *puVar6;
  uint uVar7;
  bool bVar8;
  undefined1 auStack_3c [8];
  undefined4 uStack_34;
  void *local_1c;
  undefined4 local_18;
  uint local_14;
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a178;
  local_c = ExceptionList;
  bVar8 = *(int *)((int)this + 4) == 0;
  local_18 = 0;
  local_1c = (void *)0x0;
  uVar3 = 0;
  if (!bVar8) {
    uVar3 = (uint)(*(int *)((int)this + 8) != 0);
    bVar8 = uVar3 == 0;
  }
  ExceptionList = &local_c;
  if (!bVar8) {
    ExceptionList = &local_c;
    bVar8 = FUN_004f6b50(param_1,(int *)&local_1c);
    uVar3 = CONCAT31(extraout_var,bVar8);
  }
  iVar2 = 0;
  if (uVar3 != 0) {
    iVar2 = FUN_004f6df0(local_1c,&local_18);
  }
  uVar3 = 0;
  if (iVar2 != 0) {
    uVar3 = FUN_0040d690((int)this_00);
    uStack_34 = 0x40c67d;
    FUN_0040d660(uVar3);
    uStack_34 = 0x40c688;
    puVar4 = FUN_004f62d0((int)this_00);
    local_10 = auStack_3c;
    FUN_005f2f90(auStack_3c,(int)puVar4);
    local_4 = 0;
    puVar5 = FUN_004025b0(this_00,&local_14);
    local_4._0_1_ = 1;
    puVar6 = FUN_004025b0(this_00,(uint *)&param_1);
    local_4._0_1_ = 4;
    sVar1 = (**(code **)(*this_00 + 0x30))();
    uVar3 = FUN_0040c4a0(puVar5);
    uVar7 = FUN_0040c490(puVar6);
    iVar2 = FUN_005961e0(uVar7,uVar3,sVar1);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar3 = (uint)(iVar2 != 0);
    if (uVar3 != 0) {
      FUN_004f7f60(this_00,iVar2);
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

