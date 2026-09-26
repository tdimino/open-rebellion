
undefined4 __thiscall FUN_004a90d0(void *this,int param_1)

{
  int iVar1;
  bool bVar2;
  int *piVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  uint *puVar6;
  int *piVar7;
  int iVar8;
  char *pcVar9;
  uint uVar10;
  uint local_74;
  int *local_70;
  uint uStack_6c;
  uint uStack_68;
  undefined1 auStack_64 [4];
  void *pvStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  RECT RStack_4c;
  RECT RStack_3c;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638173;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_006037f0(7);
  FUN_004ece30(&local_74);
  local_4 = 0;
  piVar3 = FUN_004a25c0(this);
  iVar8 = param_1;
  local_70 = piVar3;
  if (piVar3 == (int *)0x0) {
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar4 = 0;
  }
  else if (*(int *)((int)this + 0x158) == param_1) {
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar4 = 0;
  }
  else {
    FUN_005f5b20(*(void **)((int)this + 0x168));
    (**(code **)(*(int *)((int)this + 0x114) + 4))();
    pvStack_60 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 1;
    if (pvStack_60 == (void *)0x0) {
      puVar5 = (undefined4 *)0x0;
    }
    else {
      puVar5 = FUN_004acd80(pvStack_60,(undefined4 *)((int)this + 0x144),0);
    }
    local_4._0_1_ = 0;
    FUN_004acba0((void *)((int)this + 0x114),puVar5,0);
    switch(iVar8) {
    case 1:
      *(uint *)(*(int *)((int)this + 0x164) + 0xf4) =
           *(uint *)(*(int *)((int)this + 0x164) + 0xf4) & 0xfffdffff | 0x80000;
      FUN_00536e20(auStack_2c,piVar3,3,*(undefined4 *)((int)this + 0x148));
      local_4._0_1_ = 10;
      FUN_004ece30(&pvStack_60);
      local_4 = CONCAT31(local_4._1_3_,0xb);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        piVar3 = (int *)FUN_0052bed0((int)auStack_2c);
        puVar6 = FUN_0042d170(piVar3,auStack_64);
        local_4._0_1_ = 0xc;
        iVar8 = FUN_004ece60(puVar6);
        if ((iVar8 == 0) || ((piVar3[0x1e] & 0x100U) != 0)) {
          bVar2 = true;
        }
        else {
          bVar2 = false;
        }
        local_4 = CONCAT31(local_4._1_3_,0xb);
        FUN_00619730();
        if (bVar2) {
          FUN_004a9ab0(this,piVar3,*(void **)((int)this + 0x174));
        }
        FUN_005130d0((int)auStack_2c);
        piVar3 = local_70;
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 10;
      FUN_00619730();
      local_4._0_1_ = 0;
      FUN_00536ea0(auStack_2c);
      break;
    case 2:
      *(uint *)(*(int *)((int)this + 0x164) + 0xf4) =
           *(uint *)(*(int *)((int)this + 0x164) + 0xf4) & 0xfffdffff | 0x80000;
      FUN_00504cc0(auStack_2c,piVar3,3,*(undefined4 *)((int)this + 0x148));
      local_4 = CONCAT31(local_4._1_3_,9);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        piVar7 = (int *)FUN_0052bed0((int)auStack_2c);
        FUN_004a9ab0(this,piVar7,*(void **)((int)this + 0x174));
        FUN_005130d0((int)auStack_2c);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 0;
      FUN_00504d40(auStack_2c);
      break;
    case 3:
      *(uint *)(*(int *)((int)this + 0x164) + 0xf4) =
           *(uint *)(*(int *)((int)this + 0x164) + 0xf4) & 0xfffdffff | 0x80000;
      FUN_00503a50(auStack_2c,piVar3,3,*(undefined4 *)((int)this + 0x148));
      local_4 = CONCAT31(local_4._1_3_,8);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        piVar7 = (int *)FUN_0052bed0((int)auStack_2c);
        FUN_004a9ab0(this,piVar7,*(void **)((int)this + 0x174));
        FUN_005130d0((int)auStack_2c);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 0;
      FUN_00503ad0(auStack_2c);
      break;
    case 4:
      *(uint *)(*(int *)((int)this + 0x164) + 0xf4) =
           *(uint *)(*(int *)((int)this + 0x164) + 0xf4) | 0xa0000;
      FUN_00527050(auStack_2c,piVar3,3,*(undefined4 *)((int)this + 0x148));
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        piVar3 = (int *)FUN_0052bed0((int)auStack_2c);
        puVar6 = FUN_004025b0(piVar3,&uStack_68);
        local_4._0_1_ = 6;
        FUN_004f26d0(&local_74,puVar6);
        local_4._0_1_ = 5;
        FUN_00619730();
        RStack_3c.left = 0x24;
        RStack_3c.top = 0x25;
        uVar10 = local_74 >> 0x18;
        local_4 = CONCAT31(local_4._1_3_,7);
        if ((uVar10 < 0x24) || (0x24 < uVar10)) {
          RStack_4c.left = 0x25;
          RStack_4c.top = 0x26;
          if ((uVar10 < 0x25) || (0x25 < uVar10)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) goto LAB_004a93dc;
          bVar2 = false;
        }
        else {
LAB_004a93dc:
          bVar2 = true;
        }
        local_4 = CONCAT31(local_4._1_3_,5);
        FUN_00619730();
        if (bVar2) {
          FUN_004a9ab0(this,piVar3,*(void **)((int)this + 0x174));
        }
        FUN_005130d0((int)auStack_2c);
        piVar3 = local_70;
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 0;
      FUN_005270d0(auStack_2c);
      break;
    case 5:
      *(uint *)(*(int *)((int)this + 0x164) + 0xf4) =
           *(uint *)(*(int *)((int)this + 0x164) + 0xf4) | 0xa0000;
      FUN_00527050(auStack_2c,piVar3,3,*(undefined4 *)((int)this + 0x148));
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        piVar3 = (int *)FUN_0052bed0((int)auStack_2c);
        puVar6 = FUN_004025b0(piVar3,&uStack_6c);
        local_4._0_1_ = 3;
        FUN_004f26d0(&local_74,puVar6);
        local_4._0_1_ = 2;
        FUN_00619730();
        uStack_54 = 0x22;
        uStack_50 = 0x23;
        uVar10 = local_74 >> 0x18;
        local_4 = CONCAT31(local_4._1_3_,4);
        if ((uVar10 < 0x22) || (0x22 < uVar10)) {
          uStack_5c = 0x23;
          uStack_58 = 0x24;
          if ((uVar10 < 0x23) || (0x23 < uVar10)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) goto LAB_004a92ad;
          bVar2 = false;
        }
        else {
LAB_004a92ad:
          bVar2 = true;
        }
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_00619730();
        if (bVar2) {
          FUN_004a9ab0(this,piVar3,*(void **)((int)this + 0x174));
        }
        FUN_005130d0((int)auStack_2c);
        piVar3 = local_70;
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 0;
      FUN_005270d0(auStack_2c);
    }
    iVar8 = *(int *)(*(int *)((int)this + 0x160) + 0x94);
    if (iVar8 == 0) {
      uVar10 = 0;
    }
    else {
      uVar10 = *(uint *)(iVar8 + 0x24);
    }
    iVar8 = FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),uVar10);
    pcVar9 = (char *)FUN_00602830(iVar8);
    FUN_00601aa0(*(void **)((int)this + 0x16c),pcVar9);
    iVar8 = *(int *)((int)this + 0x16c);
    RStack_3c.left = *(LONG *)(iVar8 + 0x14);
    RStack_3c.top = *(LONG *)(iVar8 + 0x18);
    RStack_3c.right = *(LONG *)(iVar8 + 0x1c);
    RStack_3c.bottom = *(LONG *)(iVar8 + 0x20);
    InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_3c,0);
    FUN_0060a280(*(void **)((int)this + 0x164));
    iVar8 = param_1;
    if ((param_1 == 2) && (uVar10 = FUN_0041cdb0(), ((uint)piVar3[9] >> 6 & 3) == uVar10)) {
      param_1 = CONCAT22(DAT_0065d424,0x1947);
      FUN_005f2fc0(&RStack_3c,&param_1);
      local_4._0_1_ = 0xd;
      FUN_005f31a0(&RStack_3c,piVar3[0x20]);
      pcVar9 = (char *)FUN_00583c40((int)&RStack_3c);
      FUN_00601aa0(*(void **)((int)this + 0x170),pcVar9);
      iVar1 = *(int *)((int)this + 0x170);
      RStack_4c.left = *(LONG *)(iVar1 + 0x14);
      RStack_4c.top = *(LONG *)(iVar1 + 0x18);
      RStack_4c.right = *(LONG *)(iVar1 + 0x1c);
      RStack_4c.bottom = *(LONG *)(iVar1 + 0x20);
      InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_4c,0);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(&RStack_3c.left);
    }
    else {
      FUN_00601aa0(*(void **)((int)this + 0x170),(char *)&lpString_006a7f40);
      iVar1 = *(int *)((int)this + 0x170);
      RStack_3c.left = *(LONG *)(iVar1 + 0x14);
      RStack_3c.top = *(LONG *)(iVar1 + 0x18);
      RStack_3c.right = *(LONG *)(iVar1 + 0x1c);
      RStack_3c.bottom = *(LONG *)(iVar1 + 0x20);
      InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_3c,0);
    }
    *(int *)((int)this + 0x158) = iVar8;
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar4 = 1;
  }
  ExceptionList = local_c;
  return uVar4;
}

