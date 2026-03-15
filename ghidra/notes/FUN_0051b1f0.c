
uint __thiscall FUN_0051b1f0(void *this,int param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  void *this_00;
  int *this_01;
  uint uVar4;
  undefined4 uVar5;
  undefined3 extraout_var;
  int iVar6;
  int *piVar7;
  uint local_a4;
  uint local_a0;
  int local_9c;
  int local_98;
  void *local_94;
  int local_90;
  uint uStack_8c;
  uint local_88;
  uint local_84;
  undefined4 local_80;
  uint local_7c;
  undefined4 local_78;
  uint local_74;
  undefined4 local_70;
  undefined4 local_6c [4];
  uint local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642f46;
  pvStack_c = ExceptionList;
  if (param_1 == 1) {
    local_a0 = 1;
  }
  else if (param_1 == 2) {
    local_a0 = 2;
  }
  else {
    local_a0 = 0;
  }
  ExceptionList = &pvStack_c;
  local_94 = this;
  iVar2 = FUN_00506f30(param_1);
  local_98 = 0;
  local_a4 = (uint)(iVar2 != 0);
  if (iVar2 != 0) {
    iVar2 = *(int *)(iVar2 + 0x58) - *(int *)(iVar2 + 0x5c);
    if (param_1 == 2) {
      local_a4 = FUN_0055d6d0(iVar2,*(int *)((int)this + 0x5c),&local_98);
    }
    else {
      local_a4 = FUN_0055d670(iVar2,*(int *)((int)this + 0x5c),&local_98);
    }
  }
  do {
    local_9c = 0;
    FUN_004f3f70(local_6c);
    local_4 = 0;
    if (param_1 == 2) {
      uVar3 = FUN_0055d780(&local_9c,local_6c);
      if ((uVar3 == 0) || (local_a4 == 0)) {
LAB_0051b2fc:
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
    else {
      uVar3 = FUN_0055d730(&local_9c,local_6c);
      if ((uVar3 == 0) || (local_a4 == 0)) goto LAB_0051b2fc;
      bVar1 = true;
    }
    local_90 = 0;
    iVar2 = FUN_0051be20(local_9c,(int)local_6c,&local_90);
    if ((iVar2 == 0) || (local_a4 = 1, !bVar1)) {
      local_a4 = 0;
    }
    uVar3 = local_a4;
    local_98 = local_98 - local_90;
    local_88 = (uint)(-1 < local_98);
    if (local_88 != 0) {
      iVar2 = 0;
      this_00 = (void *)FUN_00505ef0();
      while (this_00 != (void *)0x0) {
        local_74 = 0x90;
        local_70 = 0x98;
        iVar2 = iVar2 + (uint)((*(uint *)((int)this_00 + 0x24) >> 6 & 3) == local_a0);
        local_4._1_3_ = (uint3)(local_4 >> 8);
        local_4._0_1_ = 1;
        this_00 = (void *)FUN_004f6010(this_00,&local_74,1);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
      iVar2 = FUN_0053e290(iVar2 + -1);
      this_01 = (int *)FUN_00505ef0();
      while ((this_01 != (int *)0x0 && (((uint)this_01[9] >> 6 & 3) != local_a0))) {
        local_5c = 0x90;
        local_58 = 0x98;
        local_4._1_3_ = (uint3)(local_4 >> 8);
        local_4._0_1_ = 2;
        this_01 = (int *)FUN_004f6010(this_01,&local_5c,1);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
      iVar6 = 0;
      if (this_01 == (int *)0x0) {
LAB_0051b4c9:
        local_a4 = 0;
      }
      else {
        do {
          if (iVar2 <= iVar6) break;
          local_7c = 0x90;
          local_78 = 0x98;
          local_4._0_1_ = 3;
          this_01 = (int *)FUN_004f6010(this_01,&local_7c,1);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
          while ((this_01 != (int *)0x0 && (((uint)this_01[9] >> 6 & 3) != local_a0))) {
            local_84 = 0x90;
            local_80 = 0x98;
            local_4._1_3_ = (uint3)(local_4 >> 8);
            local_4._0_1_ = 4;
            this_01 = (int *)FUN_004f6010(this_01,&local_84,1);
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
          }
          iVar6 = iVar6 + 1;
        } while (this_01 != (int *)0x0);
        if ((this_01 == (int *)0x0) || (local_a4 = 1, uVar3 == 0)) goto LAB_0051b4c9;
      }
      uVar3 = local_a4;
      if (this_01 != (int *)0x0) {
        iVar2 = thunk_FUN_005f5060((int)local_6c);
        if (iVar2 != 0) {
          local_54 = 0x14;
          uVar4 = *(uint *)(iVar2 + 0x24) >> 0x18;
          local_50 = 0x1c;
          if ((uVar4 < 0x14) || (0x1b < uVar4)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            FUN_004ffef0(local_4c,this_01,4,local_a0);
            local_4 = CONCAT31(local_4._1_3_,5);
            piVar7 = (int *)0x0;
            FUN_00513120((int)local_4c);
            while (local_30 != 0) {
              if (piVar7 != (int *)0x0) goto LAB_0051b610;
              uVar5 = FUN_0052bed0((int)local_4c);
              FUN_00502d10(local_2c,uVar5);
              local_4 = CONCAT31(local_4._1_3_,6);
              bVar1 = FUN_005131b0((int)local_2c);
              if (CONCAT31(extraout_var,bVar1) != 0) {
                piVar7 = (int *)FUN_0052bed0((int)local_4c);
              }
              local_4 = CONCAT31(local_4._1_3_,5);
              FUN_00502eb0(local_2c);
              FUN_005130d0((int)local_4c);
            }
            if (piVar7 == (int *)0x0) {
              uStack_8c = 0x8000004;
              local_4._1_3_ = (uint3)(local_4 >> 8);
              local_4._0_1_ = 7;
              piVar7 = FUN_004f7d50(this_01,&uStack_8c,local_a0,param_2,1,1);
              local_4 = CONCAT31(local_4._1_3_,5);
              FUN_00619730();
              if ((piVar7 == (int *)0x0) || (local_a4 = 1, uVar3 == 0)) {
                local_a4 = 0;
              }
              if (piVar7 != (int *)0x0) goto LAB_0051b610;
            }
            else {
LAB_0051b610:
              uVar3 = local_a4;
              iVar2 = FUN_0051bf30(piVar7,local_9c,(int)local_6c,param_2);
              if ((iVar2 == 0) || (local_a4 = 1, uVar3 == 0)) {
                local_a4 = 0;
              }
            }
            local_4 = local_4 & 0xffffff00;
            FUN_004fff70(local_4c);
            goto LAB_0051b68a;
          }
        }
        iVar2 = FUN_0051bf30(this_01,local_9c,(int)local_6c,param_2);
        if ((iVar2 == 0) || (local_a4 = 1, uVar3 == 0)) {
          local_a4 = 0;
        }
      }
    }
LAB_0051b68a:
    local_4 = 0xffffffff;
    FUN_004f3fb0(local_6c);
    if (local_88 == 0) {
      ExceptionList = pvStack_c;
      return local_a4;
    }
  } while( true );
}

