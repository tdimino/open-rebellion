
uint __thiscall FUN_0040c200(void *this,int *param_1)

{
  int *this_00;
  undefined3 extraout_var;
  int iVar1;
  int iVar2;
  uint uVar3;
  undefined *puVar4;
  uint *puVar5;
  uint *puVar6;
  uint uVar7;
  bool bVar8;
  undefined1 auStack_68 [8];
  undefined4 uStack_60;
  void *local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  uint local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a120;
  pvStack_c = ExceptionList;
  local_18 = 0;
  local_1c = (undefined1 *)0x0;
  local_20 = (void *)0x0;
  bVar8 = false;
  if (*(int *)((int)this + 4) != 0) {
    bVar8 = *(int *)((int)this + 8) != 0;
  }
  iVar2 = 0;
  ExceptionList = &pvStack_c;
  if (bVar8) {
    ExceptionList = &pvStack_c;
    bVar8 = FUN_004f6b50(param_1,(int *)&local_20);
    iVar2 = CONCAT31(extraout_var,bVar8);
  }
  iVar1 = 0;
  if ((iVar2 == 0) || (iVar1 = FUN_004f6df0(local_20,&local_18), iVar1 == 0)) goto LAB_0040c383;
  if ((this_00[9] & 0xc0U) == 0x40) {
    FUN_004025b0(this_00,(uint *)&param_1);
    FUN_00619730();
    FUN_004025b0(this_00,(uint *)&param_1);
LAB_0040c320:
    FUN_00619730();
  }
  else if ((this_00[9] & 0xc0U) == 0x80) {
    FUN_004025b0(this_00,(uint *)&param_1);
    FUN_00619730();
    FUN_004025b0(this_00,(uint *)&param_1);
    goto LAB_0040c320;
  }
  local_14[0] = 0x30;
  local_14[1] = 0x38;
  local_4 = 0;
  (**(code **)(*this_00 + 4))();
  local_4 = 0xffffffff;
  FUN_00619730();
  local_1c = (undefined1 *)(uint)((short)this_00[0x25] != 0);
LAB_0040c383:
  uVar3 = 0;
  if (iVar1 != 0) {
    (**(code **)(*this_00 + 0x30))();
    iVar2 = (**(code **)(*this_00 + 0x1f0))();
    FUN_0040d680(iVar2);
    iVar2 = (**(code **)(*this_00 + 500))();
    FUN_0040d670(iVar2);
    uVar3 = FUN_0040d690((int)this_00);
    uStack_60 = 0x40c3dd;
    FUN_0040d660(uVar3);
    uStack_60 = 0x40c3e8;
    puVar4 = FUN_004f62d0((int)this_00);
    local_1c = auStack_68;
    FUN_005f2f90(auStack_68,(int)puVar4);
    local_4 = 1;
    puVar5 = FUN_004025b0(this_00,local_14);
    local_4._0_1_ = 2;
    puVar6 = FUN_004025b0(this_00,(uint *)&param_1);
    local_4._0_1_ = 5;
    uVar3 = FUN_0040c4a0(puVar5);
    uVar7 = FUN_0040c490(puVar6);
    iVar2 = FUN_00596340(uVar7,uVar3);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar3 = (uint)(iVar2 != 0);
    if (uVar3 != 0) {
      FUN_004f7f60(this_00,iVar2);
    }
  }
  ExceptionList = pvStack_c;
  return uVar3;
}

