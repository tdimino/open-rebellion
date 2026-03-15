
void __thiscall FUN_00450770(void *this,int param_1)

{
  undefined1 uVar1;
  uint *puVar2;
  int *this_00;
  char *pcVar3;
  int *piVar4;
  int *piVar5;
  undefined *puVar6;
  void *pvVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  undefined4 uVar11;
  undefined4 local_e8;
  undefined1 *local_e4;
  uint local_e0;
  undefined4 local_dc [3];
  undefined1 *puStack_d0;
  undefined4 local_cc [7];
  int local_b0;
  undefined4 local_ac [7];
  int local_90;
  undefined4 local_8c [8];
  undefined4 local_6c [8];
  undefined4 local_4c [8];
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fbb4;
  pvStack_c = ExceptionList;
  pvVar7 = (void *)0x0;
  local_e4 = (undefined1 *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_e0);
  local_4 = 0;
  local_e8 = (undefined1 *)FUN_00618b70(0x74);
  local_4._0_1_ = 1;
  if (local_e8 != (undefined1 *)0x0) {
    pvVar7 = FUN_004421d0(local_e8,&local_e0);
  }
  local_4._0_1_ = 0;
  puVar2 = (uint *)FUN_00451110(*(void **)((int)this + 0x114),&local_e8);
  local_4._0_1_ = 2;
  this_00 = FUN_004f3220(*(int *)(*(int *)((int)this + 0x118) + 0x9c),puVar2);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f2f50(local_dc);
  local_4._0_1_ = 3;
  uVar1 = (undefined1)local_4;
  local_4._0_1_ = 3;
  if (param_1 == 0) {
    local_e8 = (undefined1 *)CONCAT22(DAT_0065d424,0x1925);
    FUN_005f3010(local_dc,&local_e8);
    pcVar3 = (char *)FUN_00583c40((int)local_dc);
    FUN_005f35e0((void *)((int)pvVar7 + 0x14),pcVar3);
    *(undefined4 *)((int)pvVar7 + 0x38) = 0xb;
    *(uint *)((int)pvVar7 + 0x3c) = *(uint *)((int)pvVar7 + 0x3c) | 4;
    *(undefined4 *)((int)pvVar7 + 0x50) = 1;
    FUN_005f59f0(*(void **)((int)this + 0x130),(int)pvVar7);
    uVar8 = 0x450899;
    piVar4 = this_00;
    FUN_00539d70(local_cc,this_00,1);
    iVar10 = 0;
    local_4._0_1_ = 4;
    local_e4 = &stack0xfffffefc;
    FUN_004510f0(&stack0xfffffefc,0,0);
    FUN_00450ce0(this,(uint)local_cc,uVar8,(uint)piVar4,iVar10);
    uVar8 = 0x4508cd;
    piVar4 = this_00;
    FUN_00504c40(local_ac,this_00,1);
    iVar10 = 0;
    local_4._0_1_ = 5;
    local_e4 = &stack0xfffffefc;
    FUN_004510f0(&stack0xfffffefc,0,0);
    FUN_00450ce0(this,(uint)local_ac,uVar8,(uint)piVar4,iVar10);
    uVar8 = 0x450901;
    FUN_00536da0(local_8c,this_00,1);
    iVar10 = 0;
    local_4._0_1_ = 6;
    local_e4 = &stack0xfffffefc;
    FUN_004510f0(&stack0xfffffefc,0,0);
    FUN_00450ce0(this,(uint)local_8c,uVar8,(uint)this_00,iVar10);
    local_4._0_1_ = 5;
    FUN_00536ea0(local_8c);
    local_4._0_1_ = 4;
    FUN_00504d40(local_ac);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_00539e70(local_cc);
  }
  else {
    if (param_1 == 1) {
      local_e8 = (undefined1 *)CONCAT22(DAT_0065d424,0x1923);
    }
    else {
      local_e8 = (undefined1 *)CONCAT22(DAT_0065d424,0x1924);
    }
    local_4._0_1_ = uVar1;
    FUN_005f3010(local_dc,&local_e8);
    pcVar3 = (char *)FUN_00583c40((int)local_dc);
    FUN_005f35e0((void *)((int)pvVar7 + 0x14),pcVar3);
    *(undefined4 *)((int)pvVar7 + 0x38) = 0xb;
    *(undefined4 *)((int)pvVar7 + 0x50) = 1;
    *(uint *)((int)pvVar7 + 0x3c) = *(uint *)((int)pvVar7 + 0x3c) | 4;
    FUN_005f59f0(*(void **)((int)this + 0x130),(int)pvVar7);
    FUN_004ffef0(local_cc,this_00,1,param_1);
    local_4 = CONCAT31(local_4._1_3_,7);
    FUN_00513120((int)local_cc);
    while (local_b0 != 0) {
      pvVar7 = (void *)0x0;
      iVar10 = 0;
      uVar9 = 5;
      piVar4 = (int *)FUN_0052bed0((int)local_cc);
      FUN_00450e10(this,piVar4,uVar9,iVar10,pvVar7);
      uVar11 = 1;
      uVar9 = FUN_0052bed0((int)local_cc);
      FUN_00502db0(local_ac,uVar9,uVar11);
      local_4 = CONCAT31(local_4._1_3_,8);
      FUN_00513120((int)local_ac);
      while (local_90 != 0) {
        piVar5 = (int *)FUN_0052bed0((int)local_ac);
        FUN_00450e10(this,piVar5,5,0,(void *)0x1);
        uVar8 = 0x450a6b;
        piVar4 = piVar5;
        FUN_005039d0(local_2c,piVar5,1);
        iVar10 = 1;
        local_4._0_1_ = 9;
        local_e8 = &stack0xfffffefc;
        FUN_004510f0(&stack0xfffffefc,0,0);
        FUN_00450ce0(this,(uint)local_2c,uVar8,(uint)piVar4,iVar10);
        uVar8 = 0x450aa5;
        piVar4 = piVar5;
        FUN_00504c40(local_4c,piVar5,1);
        iVar10 = 1;
        local_4._0_1_ = 10;
        local_e8 = &stack0xfffffefc;
        FUN_004510f0(&stack0xfffffefc,0,0);
        FUN_00450ce0(this,(uint)local_4c,uVar8,(uint)piVar4,iVar10);
        uVar8 = 0x450ade;
        FUN_00536da0(local_6c,piVar5,1);
        iVar10 = 1;
        local_4._0_1_ = 0xb;
        local_e8 = &stack0xfffffefc;
        FUN_004510f0(&stack0xfffffefc,0,0);
        FUN_00450ce0(this,(uint)local_6c,uVar8,(uint)piVar5,iVar10);
        local_4._0_1_ = 10;
        FUN_00536ea0(local_6c);
        local_4._0_1_ = 9;
        FUN_00504d40(local_4c);
        local_4 = CONCAT31(local_4._1_3_,8);
        FUN_00503ad0(local_2c);
        FUN_005130d0((int)local_ac);
      }
      local_4._1_3_ = (undefined3)(local_4 >> 8);
      local_4 = CONCAT31(local_4._1_3_,7);
      FUN_00502eb0(local_ac);
      FUN_005130d0((int)local_cc);
    }
    piVar4 = this_00;
    FUN_00503a50(local_8c,this_00,1,param_1);
    local_4._0_1_ = 0xc;
    iVar10 = FUN_00513180((int)local_8c);
    if (iVar10 != 0) {
      pvVar7 = (void *)FUN_00618b70(0x74);
      local_4._0_1_ = 0xd;
      puStack_d0 = (undefined1 *)pvVar7;
      if (pvVar7 == (void *)0x0) {
        pvVar7 = (void *)0x0;
      }
      else {
        puVar2 = FUN_004025b0(this_00,&local_e8);
        local_e4 = (undefined1 *)0x1;
        local_4 = CONCAT31(local_4._1_3_,0xe);
        pvVar7 = FUN_004421d0(pvVar7,puVar2);
      }
      local_4 = 0xd;
      if (((uint)local_e4 & 1) != 0) {
        FUN_00619730();
      }
      local_4._0_1_ = 0xc;
      puVar6 = FUN_004f62d0((int)this_00);
      pcVar3 = (char *)FUN_00583c40((int)puVar6);
      FUN_005f35e0((void *)((int)pvVar7 + 0x14),pcVar3);
      *(undefined4 *)((int)pvVar7 + 0x38) = 5;
      *(uint *)((int)pvVar7 + 0x3c) = *(uint *)((int)pvVar7 + 0x3c) | 4;
      *(undefined4 *)((int)pvVar7 + 0x50) = 1;
      uVar8 = 0x450c50;
      FUN_005f59f0(*(void **)((int)this + 0x130),(int)pvVar7);
      iVar10 = 0;
      puStack_d0 = &stack0xfffffefc;
      FUN_004510f0(&stack0xfffffefc,0,0);
      FUN_00450ce0(this,(uint)local_8c,(uint)piVar4,uVar8,iVar10);
    }
    local_4._0_1_ = 7;
    FUN_00503ad0(local_8c);
    local_4 = CONCAT31(local_4._1_3_,3);
    FUN_004fff70(local_cc);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_005f2ff0(local_dc);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

